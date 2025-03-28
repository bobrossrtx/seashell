use std::process::Command;
use std::path::Path;
use crate::utils::logger::{log_message, MessageType as LoggerMessageType};
use crate::utils::environment;
use crate::parser;

pub fn execute_unix_command(command: &parser::Command) -> Result<(), String> {
    log_message(LoggerMessageType::Debug, &format!("Executing Unix command: {} with args: {:?}", command.name, command.args)).ok();

    // Check if the command is empty
    if command.name.is_empty() {
        return Ok(());
    }

    let env_path = environment::get_var_as_string("PATH")
        .unwrap_or_else(|_| "/bin:/usr/bin".to_string());
    let paths: Vec<&str> = env_path.split(':').collect();

    for path in paths {
        let full_path = Path::new(path).join(&command.name);
        if full_path.exists() && full_path.is_file() {
            let output = Command::new(full_path)
                .args(&command.args)
                .output()
                .map_err(|e| format!("Failed to execute '{}': {}", command.name, e))?;

            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            return Ok(());
        }
    }

    Err(format!("Command '{}' not found in PATH", command.name))
}