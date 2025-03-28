use std::collections::HashMap;
use crate::commands;

pub fn initialize_command_map() -> HashMap<&'static str, fn(&[String]) -> Result<(), String>> {
    let mut command_map: HashMap<&str, fn(&[String]) -> Result<(), String>> = HashMap::new();
    command_map.insert("cd", |args| commands::cd::CdCommand::new().execute(args));
    command_map.insert("pwd", |args| commands::pwd::PwdCommand::new().execute(args));
    command_map.insert("ls", |args| commands::ls::LsCommand::new().execute(args));
    command_map.insert("env", |args| commands::env::EnvListCommand::new().execute(args));
    command_map.insert("set", |args| commands::env::EnvSetCommand::new().execute(args));
    command_map.insert("get", |args| commands::env::EnvGetCommand::new().execute(args));
    command_map.insert("unset", |args| commands::env::EnvUnsetCommand::new().execute(args));
    command_map
}