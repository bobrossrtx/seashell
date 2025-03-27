use crate::utils::command_metadata::CommandMetadata;
use crate::utils::arg_parser::{ArgDefinition, ArgParser};
use crate::utils::environment::list_vars;

pub struct EnvListCommand {
    pub metadata: CommandMetadata,
}

impl EnvListCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "env",
                "List all environment variables",
                "1.0.0",
                "env",
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

        let vars = list_vars()?;
        for (key, value) in vars {
            println!("{}={}", key, value);
        }
        Ok(())
    }
}