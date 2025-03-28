use crate::utils::environment::set_var;
use crate::utils::arg_parser::{ArgDefinition, ArgParser};
use crate::utils::command_metadata::CommandMetadata;

pub struct PwdCommand {
    pub metadata: CommandMetadata,
}

impl PwdCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "pwd",
                "Print the current working directory",
                "1.0.0",
                "pwd",
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

        let current_dir = std::env::current_dir()
            .map_err(|e| format!("pwd: failed to get current directory: {}", e))?;
        
        // Set the PWD environment variable
        if let Err(e) = set_var("PWD", current_dir.to_str().unwrap_or_default()) {
            return Err(format!("pwd: failed to set PWD environment variable: {}", e));
        }

        println!("{}", current_dir.display());
        Ok(())
    }
}