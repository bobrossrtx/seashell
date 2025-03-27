use crate::utils::command_metadata::CommandMetadata;
use crate::utils::arg_parser::{ArgDefinition, ArgParser};
use crate::utils::environment::set_var;

pub struct EnvSetCommand {
    pub metadata: CommandMetadata,
}

impl EnvSetCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "set",
                "Set an environment variable",
                "1.0.0",
                "set [KEY] [VALUE]",
            ),
        }
    }

    pub fn execute(&mut self, args: &[String]) -> Result<(), String> {
        let definitions = [
            ArgDefinition {
                flag: Some("h".to_string()),
                alias: Some("help".to_string()),
                help: "Display this help message".to_string(),
            },
        ];

        let mut parser = ArgParser::new();
        parser.parse(args, &definitions, &mut self.metadata);

        if parser.is_flag_set("help") {
            self.metadata.display_help();
            return Ok(());
        }

        let key = parser.positional.get(0).ok_or("set: missing key")?;
        let value = parser.positional.get(1).ok_or("set: missing value")?;

        set_var(key, value)
    }
}