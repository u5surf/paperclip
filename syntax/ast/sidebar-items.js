initSidebarItems({"constant":[["CRATE_NODE_ID","`NodeId` used to represent the root of the crate."],["DUMMY_NODE_ID","When parsing and doing expansions, we initially give all AST nodes this AST node value. Then later, in the renumber pass, we renumber them to have small, positive ids."]],"enum":[["AsmDialect","Inline assembly dialect."],["AttrStyle","Distinguishes between `Attribute`s that decorate items and Attributes that are contained as statements within items. These two cases need to be distinguished for pretty-printing."],["BinOpKind",""],["BindingMode",""],["BlockCheckMode",""],["CaptureBy","A capture clause."],["Constness",""],["CrateSugar",""],["Defaultness",""],["ExprKind",""],["FloatTy",""],["ForeignItemKind","An item within an `extern` block."],["FunctionRetTy",""],["GenericArg",""],["GenericArgs","The arguments of a path segment."],["GenericBound","The AST represents all type param bounds as types. `typeck::collect::compute_bounds` matches these against the \"special\" built-in traits (see `middle::lang_items`) and detects `Copy`, `Send` and `Sync`."],["GenericParamKind",""],["Guard",""],["ImplItemKind",""],["ImplPolarity",""],["IntTy",""],["IsAsync",""],["IsAuto","Is the trait definition an auto trait?"],["ItemKind",""],["LitIntType",""],["LitKind","Literal kind."],["MacDelimiter",""],["MacStmtStyle",""],["MetaItemKind","A compile-time attribute item."],["Movability","The movability of a generator / closure literal."],["Mutability",""],["NestedMetaItemKind","Possible values inside of compile-time attribute lists."],["ParamKindOrd","Specifies the enforced ordering for generic parameters. In the future, if we wanted to relax this order, we could override `PartialEq` and `PartialOrd`, to allow the kinds to be unordered."],["PatKind",""],["RangeEnd",""],["RangeLimits","Limit types of a range (inclusive or exclusive)"],["RangeSyntax",""],["SelfKind","Alternative representation for `Arg`s describing `self` parameter of methods."],["StmtKind",""],["StrStyle",""],["TraitBoundModifier","A modifier on a bound, currently this is only used for `?Sized`, where the modifier is `Maybe`. Negative bounds should also be handled here."],["TraitItemKind",""],["TraitObjectSyntax","Syntax used to declare a trait object."],["TyKind","The various kinds of type recognized by the compiler."],["UintTy",""],["UnOp",""],["UnsafeSource",""],["Unsafety",""],["UseTreeKind","Part of `use` item to the right of its prefix."],["VariantData","Fields and Ids of enum variants and structs"],["VisibilityKind",""],["WherePredicate","A single predicate in a where-clause."]],"struct":[["AngleBracketedArgs","A path like `Foo<'a, T>`."],["AnonConst","A constant (expression) that's not an item or associated item, but needs its own `DefId` for type-checking, const-eval, etc. These are usually found nested inside types (e.g., array lengths) or expressions (e.g., repeat counts), and also used to define explicit discriminant values for enum variants."],["Arg","An argument in a function header."],["Arm","An arm of a 'match'."],["AttrId",""],["Attribute","Metadata associated with an item. Doc-comments are promoted to attributes that have `is_sugared_doc = true`."],["BareFnTy",""],["Block","A Block (`{ .. }`)."],["Crate",""],["EnumDef",""],["Expr","An expression"],["Field",""],["FieldPat","A single field in a struct pattern"],["FnDecl","Header (not the body) of a function declaration."],["FnHeader","A function header."],["ForeignItem",""],["ForeignMod","Foreign module declaration."],["GenericParam",""],["Generics","Represents lifetime, type and const parameters attached to a declaration of a function, enum, trait, etc."],["GlobalAsm","Global inline assembly."],["Ident",""],["ImplItem",""],["InlineAsm","Inline assembly."],["InlineAsmOutput","Inline assembly."],["Item","An item."],["Label",""],["Lifetime",""],["Local","Local represents a `let` statement, e.g., `let <pat>:<ty> = <expr>;`."],["Mac_","Represents a macro invocation. The `Path` indicates which macro is being invoked, and the vector of token-trees contains the source of the macro invocation."],["MacroDef",""],["MetaItem","A spanned compile-time attribute item."],["MethodSig","Represents a method's signature in a trait declaration, or in an implementation."],["Mod","Module declaration."],["MutTy",""],["Name","A symbol is an interned or gensymed string. The use of `newtype_index!` means that `Option<Symbol>` only takes up 4 bytes, because `newtype_index! reserves the last 256 values for tagging purposes."],["NodeId",""],["ParenthesizedArgs","A path like `Foo(A, B) -> C`."],["Pat",""],["Path","A \"Path\" is essentially Rust's notion of a name."],["PathSegment","A segment of a path: an identifier, an optional lifetime, and a set of types."],["PolyTraitRef",""],["QSelf","The explicit `Self` type in a \"qualified path\". The actual path, including the trait and the associated item, is stored separately. `position` represents the index of the associated item qualified with this `Self` type."],["Stmt","A statement"],["StructField","Field of a struct."],["TraitItem","Represents an item declaration within a trait declaration, possibly including a default implementation. A trait item is either required (meaning it doesn't have an implementation, just a signature) or provided (meaning it has a default implementation)."],["TraitRef","`TraitRef`s appear in impls."],["Ty",""],["TypeBinding",""],["UseTree","A tree of paths sharing common prefixes. Used in `use` items both at top-level and inside of braces in import groups."],["Variant_",""],["WhereBoundPredicate","A type bound."],["WhereClause","A where-clause in a definition."],["WhereEqPredicate","An equality predicate (unsupported)."],["WhereRegionPredicate","A lifetime predicate."]],"type":[["BinOp",""],["CrateConfig","The set of `MetaItem`s that define the compilation environment of the crate, used to drive conditional compilation."],["ExplicitSelf",""],["GenericBounds",""],["Lit","A literal."],["Mac",""],["NestedMetaItem","A spanned compile-time attribute list item."],["SpannedIdent",""],["Variant",""],["Visibility",""]]});