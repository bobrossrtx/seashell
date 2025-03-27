use std::collections::HashMap;
use std::process::Command;
use reedline::{Reedline, Signal};

mod utils; // Ensure the utils module is included
mod parser;
mod commands;

use utils::seashellprompt::SeashellPrompt;
use utils::logger::{log_message, MessageType as LoggerMessageType};


fn execute_unix_command(command: &parser::Command) -> Result<(), String> {
    log_message(LoggerMessageType::Debug, &format!("Executing Unix command: {} with args: {:?}", command.name, command.args)).ok();
    use std::path::Path;

    let env_path = utils::environment::get_var_as_string("PATH")
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

fn main() {
    if let Err(err) = utils::logger::initialize_logger() {
        eprintln!("Failed to initialize logger: {}", err);
    } else {
        log_message(LoggerMessageType::Info, "Logger initialized successfully").ok();
    }

    utils::environment::initialize_default_vars();

    let mut line_editor = Reedline::create();
    let prompt = SeashellPrompt;

    let mut command_map: HashMap<&str, fn(&[String]) -> Result<(), String>> = HashMap::new();
    command_map.insert("cd", |args| commands::cd::CdCommand::new().execute(args));
    command_map.insert("pwd", |args| commands::pwd::PwdCommand::new().execute(args));
    command_map.insert("ls", |args| commands::ls::LsCommand::new().execute(args));
    command_map.insert("env", |args| commands::env::EnvListCommand::new().execute(args));
    command_map.insert("set", |args| commands::env::EnvSetCommand::new().execute(args));
    command_map.insert("get", |args| commands::env::EnvGetCommand::new().execute(args));
    command_map.insert("unset", |args| commands::env::EnvUnsetCommand::new().execute(args));

    
    // // Example: Add a sample command to the map
    // command_map.insert("echo", |args| {
    //     println!("{}", args.join(" "));
    //     Ok(())
    // });

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(input)) => {
                let input = input.trim();
                if input == "exit" || input == "quit" {
                    println!("Goodbye!");
                    break;
                }

                match parser::parse_command(input) {
                    Ok(command) => {
                        if let Some(executor) = command_map.get(command.name.as_str()) {
                            if let Err(err) = executor(&command.args) {
                                println!("Error: {}", err);
                            }
                        } else if let Err(err) = execute_unix_command(&command) {
                            println!("Error: {}", err);
                        }
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }
            Ok(Signal::CtrlC) => {
                println!("Caught CTRL+C. Returning to prompt.");
            }
            Ok(Signal::CtrlD) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error reading line: {}", err);
            }
        }
    }

    log_message(LoggerMessageType::Info, "Shell exited").ok();
}