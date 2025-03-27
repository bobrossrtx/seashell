use crate::utils::command_metadata::CommandMetadata;
use crate::utils::arg_parser::{ArgDefinition, ArgParser};
use crate::utils::environment::get_var;

pub struct EnvGetCommand {
    pub metadata: CommandMetadata,
}

impl EnvGetCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "get",
                "Get the value of an environment variable",
                "1.0.0",
                "get [KEY]",
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

        let key = parser.positional.get(0).ok_or("get: missing key")?;

        match get_var(key) {
            Ok(value) => {
                println!("{}", value);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}