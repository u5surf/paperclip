use super::object::{ApiObject, ObjectField, OpRequirement, Parameter};
use super::state::EmitterState;
use super::RUST_KEYWORDS;
use crate::error::PaperClipError;
use crate::v2::{
    models::{self, Api, DataType, DataTypeFormat, OperationMap},
    Schema,
};
use failure::Error;
use heck::{CamelCase, SnekCase};

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

/// Checks if the given type/format matches a known Rust type and returns it.
fn matching_unit_type(
    format: Option<&DataTypeFormat>,
    type_: Option<DataType>,
) -> Option<&'static str> {
    match format {
        Some(DataTypeFormat::Int32) => Some("i32"),
        Some(DataTypeFormat::Int64) => Some("i64"),
        Some(DataTypeFormat::Float) => Some("f32"),
        Some(DataTypeFormat::Double) => Some("f64"),
        _ => match type_ {
            Some(DataType::Integer) => Some("i64"),
            Some(DataType::Number) => Some("f64"),
            Some(DataType::Boolean) => Some("bool"),
            Some(DataType::String) => Some("String"),
            _ => None,
        },
    }
}

/// `Emitter` represents the interface for generating the relevant
/// modules, API object definitions and the associated calls.
pub trait Emitter: Sized {
    /// The associated `Schema` implementor.
    type Definition: Schema + Debug;

    /// Returns a reference to the underlying state.
    fn state(&self) -> &EmitterState;

    /// Entrypoint for emitter. Given an API spec, generate code
    /// inside Rust modules in the configured working directory.
    fn generate(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
        let gen = CodegenEmitter(self);
        // Generate file contents by accumulating definitions.
        for (name, schema) in &api.definitions {
            debug!("Creating definition {}", name);
            let schema = schema.read();
            gen.generate_def_from_root(&schema)?;
        }

        let state = gen.state();
        state.declare_modules()?;
        state.write_definitions()?;

        for (path, map) in &api.paths {
            gen.collect_requirements_for_path(path, map)?;
        }

        state.add_builders()?;
        state.add_deps()?;

        Ok(())
    }

    /// Returns an iterator of path components for the given definition.
    ///
    /// **NOTE:** All components are [snake_cased](https://docs.rs/heck/*/heck/trait.SnekCase.html)
    /// (including the definition name).
    fn def_ns_name<'a>(
        &self,
        def: &'a Self::Definition,
    ) -> Result<Box<Iterator<Item = String> + 'a>, Error> {
        let state = self.state();
        def.name()
            .map(|n| n.split(state.ns_sep).map(SnekCase::to_snek_case))
            .ok_or_else(|| {
                trace!("Invalid name for definition: {:?}", def);
                PaperClipError::InvalidDefinitionName.into()
            })
            .map(|i| Box::new(i) as Box<_>)
    }

    /// Returns the [CamelCase](https://docs.rs/heck/*/heck/trait.CamelCase.html)
    /// name for the given definition.
    fn def_name(&self, def: &Self::Definition) -> Result<String, Error> {
        Ok(self
            .def_ns_name(def)?
            .last()
            .map(|s| s.to_camel_case())
            .expect("last item always exists for split?"))
    }

    /// Returns the module path (from working directory) for the given definition.
    ///
    /// **NOTE:** This doesn't (shouldn't) set any extension to the leaf component.
    fn def_mod_path(&self, def: &Self::Definition) -> Result<PathBuf, Error> {
        let state = self.state();
        let mut path = state.working_dir.clone();
        path.extend(self.def_ns_name(def)?);
        path.set_extension("rs");
        Ok(path)
    }

    /// Builds a given definition. Also takes a `bool` to specify whether we're
    /// planning to define a Rust type or whether we're reusing an existing type.
    ///
    /// **NOTE:** We resolve type aliases to known types.
    fn build_def(&self, def: &Self::Definition, define: bool) -> Result<EmittedUnit, Error> {
        trace!("Building definition: {:?}", def);
        if let Some(ty) = matching_unit_type(def.format(), def.data_type()) {
            trace!("Matches unit type: {}", ty);
            if define {
                return Ok(EmittedUnit::None);
            }

            return Ok(EmittedUnit::Known(ty.to_owned()));
        }

        match def.data_type() {
            Some(DataType::Array) => CodegenEmitter(self).emit_array(def, define),
            Some(DataType::Object) => CodegenEmitter(self).emit_object(def, define),
            Some(_) => unreachable!("bleh?"), // we've already handled everything else
            None => {
                if define {
                    Ok(EmittedUnit::None)
                } else {
                    Ok(EmittedUnit::Known("String".into()))
                }
            }
        }
    }
}

struct CodegenEmitter<'a, E>(&'a E)
where
    Self: Sized;

impl<'a, E> Deref for CodegenEmitter<'a, E> {
    type Target = E;

    fn deref(&self) -> &E {
        &self.0
    }
}

impl<'a, E> CodegenEmitter<'a, E>
where
    E: Emitter,
{
    /// Given a schema definition, generate the corresponding Rust definition.
    ///
    /// **NOTE:** This doesn't generate any files. It only adds the generated stuff
    /// to `EmitterState`.
    fn generate_def_from_root(&self, def: &E::Definition) -> Result<(), Error> {
        let state = self.state();
        // Generate the object.
        let mut object = match self.build_def(def, true)? {
            EmittedUnit::Object(o) => o,
            // We don't care about type aliases because we resolve them anyway.
            _ => return Ok(()),
        };

        let mod_path = self.def_mod_path(def)?;
        // Create parent dirs recursively for the leaf module.
        let dir_path = mod_path
            .parent()
            .ok_or(PaperClipError::InvalidDefinitionPath(mod_path.clone()))?;
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        // Get the path without the extension.
        let full_path = dir_path.join(
            mod_path
                .file_stem()
                .ok_or(PaperClipError::InvalidDefinitionPath(mod_path.clone()))?,
        );
        // Get the relative path to the parent dir.
        let rel_path = full_path
            .strip_prefix(&state.working_dir)
            .map_err(|_| PaperClipError::InvalidDefinitionPath(full_path.clone()))?;

        // Gather the immediate parent-children pairs for module declarations.
        let mut mods = state.mod_children.borrow_mut();
        for path in rel_path.ancestors() {
            match (path.parent(), path.file_name()) {
                (Some(parent), Some(name)) if parent.parent().is_some() => {
                    let entry = mods.entry(parent.into()).or_insert_with(HashSet::new);
                    entry.insert(name.to_string_lossy().into_owned());
                }
                _ => (),
            }
        }

        // Set the path for future reference
        object.path = rel_path.to_string_lossy().into_owned().replace('/', "::");

        // Add generated object to state.
        let mut def_mods = state.def_mods.borrow_mut();
        def_mods.insert(mod_path, object);

        Ok(())
    }

    /// Given a path and an operation map, collect the stuff required
    /// for generating builders later.
    // FIXME: Cleanup before this infection spreads!
    fn collect_requirements_for_path(
        &self,
        path: &str,
        map: &OperationMap<E::Definition>,
    ) -> Result<(), Error> {
        debug!("Collecting builder requirement for {:?}", path);
        let state = self.state();

        let mut unused_params = vec![];
        // Collect all the parameters local to some API call.
        if let Some(global_params) = map.parameters.as_ref() {
            let (params, _) = self.collect_parameters(path, global_params)?;
            // FIXME: What if a body is "required" globally (for all operations)?
            // This means, operations can override the body with some other schema
            // and we may need to map it to the appropriate builders.
            unused_params = params;
        }

        // Now collect the parameters local to an API call operation (method).
        for (&meth, op) in &map.methods {
            let mut op_addressed = false;
            let mut unused_local_params = vec![];

            if let Some(local_params) = op.parameters.as_ref() {
                let (mut params, schema_path) = self.collect_parameters(path, local_params)?;
                // If we have unused params which don't exist in the method-specific
                // params (which take higher precedence), then we can copy those inside.
                for global_param in &unused_params {
                    if params
                        .iter()
                        .find(|p| p.name == global_param.name)
                        .is_none()
                    {
                        params.push(global_param.clone());
                    }
                }

                // If there's a matching object, add the params to its operation.
                if let Some(pat) = schema_path.as_ref() {
                    op_addressed = true;
                    let mut def_mods = state.def_mods.borrow_mut();
                    let obj = def_mods.get_mut(pat).expect("bleh?");
                    let ops = obj
                        .paths
                        .entry(path.into())
                        .or_insert_with(Default::default);
                    ops.req.insert(
                        meth,
                        OpRequirement {
                            id: op.operation_id.clone(),
                            params,
                            body_required: true,
                        },
                    );
                } else {
                    unused_local_params = params;
                }
            } else {
                unused_local_params = unused_params.clone();
            }

            // We haven't attached this operation to any object.
            // Let's try from the response maybe...
            if !op_addressed {
                let mut def_mods = state.def_mods.borrow_mut();
                for schema in op
                    .responses
                    .iter()
                    .filter(|(c, _)| c.starts_with('2')) // 2xx response
                    .filter_map(|(_, r)| r.schema.as_ref())
                {
                    let pat = self.def_mod_path(&*schema.read()).ok();
                    let obj = match pat.and_then(|p| def_mods.get_mut(&p)) {
                        Some(o) => o,
                        None => {
                            warn!(
                                "Skipping unknown response schema for path {:?}: {:?}",
                                path, schema
                            );
                            continue;
                        }
                    };

                    let ops = obj
                        .paths
                        .entry(path.into())
                        .or_insert_with(Default::default);
                    ops.req.insert(
                        meth,
                        OpRequirement {
                            id: op.operation_id.clone(),
                            params: unused_local_params,
                            body_required: false,
                        },
                    );

                    break;
                }
            }
        }

        // FIXME: If none of the parameters (local to operation or global) specify
        // a body then we should use something (say, `operationID`) to generate
        // a builder and forward `unused_params` to it?

        Ok(())
    }

    /// Given a bunch of resolved parameters, validate and collect a simplified version of them.
    fn collect_parameters(
        &self,
        path: &str,
        obj_params: &[models::Parameter<E::Definition>],
    ) -> Result<(Vec<Parameter>, Option<PathBuf>), Error> {
        let def_mods = self.state().def_mods.borrow();
        let mut schema_path = None;
        let mut params = vec![];
        for p in obj_params {
            p.check(path)?; // validate the parameter

            if let Some(def) = p.schema.as_ref() {
                // If a schema exists, then get its path for later use.
                let pat = self.def_mod_path(&*def.read())?;
                def_mods
                    .get(&pat)
                    .ok_or(PaperClipError::UnsupportedParameterDefinition(
                        p.name.clone(),
                        path.into(),
                    ))?;
                schema_path = Some(pat);
                continue;
            }

            // Enforce that the parameter is a known type and collect it.
            let ty = matching_unit_type(p.format.as_ref(), p.data_type).ok_or(
                PaperClipError::UnknownParameterType(p.name.clone(), path.into()),
            )?;
            params.push(Parameter {
                name: p.name.clone(),
                ty_path: ty.into(),
                required: p.required,
            });
        }

        Ok((params, schema_path))
    }

    /// Assumes that the given definition is an array and returns the corresponding
    /// vector type for it.
    fn emit_array(&self, def: &E::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if define {
            return Ok(EmittedUnit::None);
        }

        let it = def
            .items()
            .ok_or(PaperClipError::MissingArrayItem(self.def_name(def).ok()))?;

        let schema = it.read();
        let ty = self.build_def(&schema, false)?.known_type();
        Ok(EmittedUnit::Known(String::from("Vec<") + &ty + ">"))
    }

    /// Assumes that the given definition is an object and returns the corresponding
    /// Rust struct / map.
    fn emit_object(&self, def: &E::Definition, define: bool) -> Result<EmittedUnit, Error> {
        match self.try_emit_map(def, define)? {
            EmittedUnit::None => (),
            x => return Ok(x),
        }

        if !define {
            // Use absolute paths to save some pain.
            // FIXME: This assumes that the working directory is the immediate
            // child module of a crate. We should support custom prefixes.
            let mut ty_path = String::from("crate");
            let mut iter = self.def_ns_name(def)?.peekable();
            while let Some(mut c) = iter.next() {
                ty_path.push_str("::");
                if iter.peek().is_none() {
                    ty_path.push_str(&c);
                    ty_path.push_str("::");
                    c = c.to_camel_case();
                }

                ty_path.push_str(&c);
            }

            return Ok(EmittedUnit::Known(ty_path));
        }

        self.emit_struct(def)
    }

    /// Checks if the given definition is a simple map and returns the corresponding `BTreeMap`.
    fn try_emit_map(&self, def: &E::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if define {
            return Ok(EmittedUnit::None);
        }

        if let Some(s) = def.additional_properties() {
            let schema = s.read();
            let ty = self.build_def(&schema, false)?.known_type();
            let map = format!("std::collections::BTreeMap<String, {}>", ty);
            Ok(EmittedUnit::Known(map))
        } else {
            Ok(EmittedUnit::None)
        }
    }

    /// Helper for `emit_object` - This returns the Rust struct definition for the
    /// given schema definition.
    fn emit_struct(&self, def: &E::Definition) -> Result<EmittedUnit, Error> {
        let mut obj = ApiObject::with_name(self.def_name(def)?);

        if let Some(props) = def.properties() {
            props
                .iter()
                .try_for_each(|(name, prop)| -> Result<(), Error> {
                    let mut new_name = name.to_snek_case();
                    // Check if the field matches a Rust keyword and add '_' suffix.
                    if RUST_KEYWORDS.iter().any(|&k| k == new_name) {
                        new_name.push('_');
                    }

                    let schema = prop.read();
                    let ty = self.build_def(&schema, false)?;

                    obj.fields.push(ObjectField {
                        // If we've modified the name, mark it for serde renaming.
                        rename: if new_name != name.as_str() {
                            Some(name.clone())
                        } else {
                            None
                        },
                        name: new_name,
                        ty_path: ty.known_type(),
                        is_required: def.is_required_property(name),
                        boxed: schema.is_cyclic(),
                    });

                    Ok(())
                })?
        }

        Ok(EmittedUnit::Object(obj))
    }
}

/// Some "thing" emitted by the emitter.
pub enum EmittedUnit {
    /// Object represented as a Rust struct.
    Object(ApiObject),
    /// Some Rust type.
    Known(String),
    /// Nothing to do.
    None,
}

impl EmittedUnit {
    #[inline]
    fn known_type(self) -> String {
        match self {
            EmittedUnit::Known(s) => s,
            _ => panic!("Emitted unit is not a known type"),
        }
    }
}
