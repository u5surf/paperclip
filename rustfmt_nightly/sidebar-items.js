initSidebarItems({"enum":[["Color",""],["Edition",""],["EmitMode",""],["ErrorKind","The various errors that can occur during formatting. Note that not all of these can currently be propagated to clients."],["FileName","Defines the name of an input - either a file or stdin."],["Input",""],["NewlineStyle",""],["Verbosity",""]],"fn":[["load_config","Loads a config by checking the client-supplied options and if appropriate, the file system (including searching the file system for overrides)."]],"macro":[["configuration_option_enum_stringify","Macro that will stringify the enum variants or a provided textual repr"],["impl_enum_serialize_and_deserialize","Macro for deriving implementations of Serialize/Deserialize for enums"]],"struct":[["Config",""],["FileLines","A set of lines in files."],["FormatReport","Reports on any issues that occurred during a run of Rustfmt."],["FormatReportFormatter","Formats the warnings/errors in a [`FormatReport`]."],["FormatReportFormatterBuilder","A builder for [`FormatReportFormatter`]."],["ModifiedChunk","A single span of changed lines, with 0 or more removed lines and a vector of 0 or more inserted lines."],["ModifiedLines","Set of changed sections of a file."],["Range","A range that is inclusive of both ends."],["Session","A session is a run of rustfmt across a single or multiple inputs."]],"trait":[["CliOptions","Maps client-supplied options to Rustfmt's internals, mostly overriding values in a config with values from the command line."]]});