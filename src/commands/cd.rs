use std::env;

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

        let path = parser.positional.get(0).map_or_else(
            || env::var("HOME").map_err(|_| "cd: HOME not set".to_string()),
            |p| Ok(p.clone()),
        )?;
        // set the current directory with env::set_current_dir then we call change_directory
        // to update the environment variables
        env::set_current_dir(path.clone()).map_err(|e| format!("cd: {}: {}", path, e))?;
        
        let current_path = env::current_dir().map_err(|e| format!("cd: {}: {}", path, e))?;
        let current_path_str = current_path.to_str().ok_or_else(|| format!("cd: invalid UTF-8 in path: {}", path))?;
        change_directory(current_path_str).map_err(|e| format!("cd: {}: {}", path, e))?;
        Ok(())

    }
}