use crate::utils::environment;
use crate::utils::printer::{print_message, MessageType};

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if parts.is_empty() {
        print_message(MessageType::Error, "No command entered");
        return Err("No command entered".to_string());
    }

    // Expand environment variables in arguments
    for part in &mut parts {
        if part.starts_with('$') {
            let var_name = &part[1..];
            match environment::get_var(var_name) {
                Ok(_) => {
                    match environment::get_var_as_string(var_name) {
                        Ok(value) => {
                            *part = value;
                        }
                        Err(_) => {
                            return Err(format!("Failed to retrieve environment variable '{}'", var_name));
                        }
                    }
                }
                Err(_) => {
                    return Err(format!("Failed to retrieve environment variable '{}'", var_name));
                }
            }
        }
    }

    let name = parts.remove(0);
    Ok(Command { name, args: parts })
}