use crate::utils::environment::change_directory;
use crate::utils::arg_parser::{ArgDefinition, ArgParser};
use crate::utils::command_metadata::CommandMetadata;

pub struct CdCommand {
    pub metadata: CommandMetadata,
}

impl CdCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "cd",
                "Change the current directory",
                "1.0.0",
                "cd [DIRECTORY]",
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

        let path = parser.positional.get(0).ok_or("cd: missing operand")?;
        change_directory(path).map_err(|e| format!("cd: {}: {}", path, e))
    }
}