use reedline::{Reedline, Signal, FileBackedHistory};

mod utils; // Ensure the utils module is included
mod parser;
mod commands;
mod settings;

use utils::seashellprompt::SeashellPrompt;
use utils::logger::{log_message, MessageType as LoggerMessageType};
use utils::printer::Printer;
use utils::command_executor::execute_unix_command;
use commands::command_map::initialize_command_map;
use utils::seashellarguments::SeashellArguments;
use settings::GLOBAL_SETTINGS;

fn main() {
    let mut seashell_args = SeashellArguments::new();

    // Define supported arguments
    seashell_args.define_argument("debug", Some('d'), "Enable debug mode");
    seashell_args.define_argument("help", Some('h'), "Show help information");
    seashell_args.define_argument("version", None, "Show version information");

    // Parse arguments
    let parsed_args = seashell_args.parser();

    // Handle unrecognized arguments
    parsed_args.handle_unrecognized(&seashell_args);

    // Handle recognized arguments
    if parsed_args.recognized.contains(&"--help".to_string()) || parsed_args.recognized.contains(&"-h".to_string()) {
        seashell_args.print_help();
        return;
    }

    if parsed_args.recognized.contains(&"--version".to_string()) {
       seashell_args.print_version();
        return;
    }

    if parsed_args.recognized.contains(&"--debug".to_string()) || parsed_args.recognized.contains(&"-d".to_string()) {
        // Enable debug mode in the global settings
        let mut settings = GLOBAL_SETTINGS.write().unwrap();
        settings.debug_mode = true;
        settings.enable_logging = true;
        Printer::debug("Debug mode enabled");
    }

    // Initialize logger
    if let Err(err) = utils::logger::initialize_logger() {
        Printer::error(&format!("Failed to initialize seashell's log: {}", err));
    } else {
        log_message(LoggerMessageType::Info, "Logger initialized successfully").ok();
        if GLOBAL_SETTINGS.read().unwrap().enable_logging {
            log_message(LoggerMessageType::Info, "Logging is enabled").ok();
        } else {
            log_message(LoggerMessageType::Info, "Logging is disabled").ok();
        }

        if GLOBAL_SETTINGS.read().unwrap().debug_mode {
            log_message(LoggerMessageType::Debug, "Debug mode is enabled").ok();
        } else {
            log_message(LoggerMessageType::Info, "Debug mode is disabled").ok();
        }
    }

    utils::environment::initialize_default_vars();

    // Set up the history file
    let home_dir = std::env::var("HOME").expect("Failed to get HOME environment variable");
    let history_path = format!("{}/.seashell/.command_history", home_dir);
    let history = Box::new(FileBackedHistory::with_file(50, std::path::PathBuf::from(history_path)).unwrap());
    let mut line_editor = Reedline::create().with_history(history);
    let prompt = SeashellPrompt;

    let command_map = initialize_command_map();

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
                                Printer::error(&format!("Command execution failed: {}", err));
                            }
                        } else if let Err(err) = execute_unix_command(&command) {
                            Printer::error(&format!("Command execution failed: {}", err));
                        }
                    }
                    Err(err) => {
                        Printer::error(&format!("Failed to parse command: {}", err));
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
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("Goodbye!");
                    break;
                } else {
                    Printer::error(&format!("Error reading line: {}", err));
                }
            }
        }
    }

    log_message(LoggerMessageType::Info, "Shell exited").ok();
}