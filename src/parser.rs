use crate::utils::environment;
use crate::utils::printer::Printer;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if parts.is_empty() {
        return Ok(Command { name: String::new(), args: Vec::new() });
    }

    parts.retain(|part| !part.is_empty());

    // parse the command and its arguments
    // Check for special characters and handle them accordingly
    for part in &mut parts {
        if part.starts_with('$') {
            let var_name = if part.starts_with("${") && part.contains('}') {
                let end_index = part.find('}').unwrap();
                &part[2..end_index]
            } else if part.starts_with("$(") && part.ends_with(")") {
                &part[2..part.len() - 1] // To handle script parsing in future
            } else {
                &part[1..]
            };

            match environment::get_var(var_name) {
                Ok(_) => {
                    match environment::get_var_as_string(var_name) {
                        Ok(value) => {
                            *part = part.replacen(&format!("${{{}}}", var_name), &value, 1);
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
        if part.starts_with('\"') && part.ends_with('\"') {
            *part = part[1..part.len() - 1].to_string();
        } else if part.starts_with('\'') && part.ends_with('\'') {
            *part = part[1..part.len() - 1].to_string();
        }
        if part.contains('\"') || part.contains('\'') {
            return Err(format!("Unmatched quotes in command: {}", input));
        }
        if part == "~" {
            if let Ok(home) = environment::get_var_as_string("HOME") {
                *part = home;
            } else {
                Printer::error("Failed to retrieve HOME environment variable");
                *part = "~".to_string();
            }
        }
    }

    let name = parts.remove(0);
    Ok(Command { name, args: parts })
}