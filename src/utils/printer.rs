use colored::*;

pub enum MessageType {
    Error,
    Debug,
    Info,
    Warning,
}

pub fn print_message(message_type: MessageType, message: &str) {
    match message_type {
        MessageType::Error => println!("{}", format!("[ERROR]: {}", message).red()),
        MessageType::Debug => println!("{}", format!("[DEBUG]: {}", message).blue()),
        MessageType::Info => println!("{}", format!("[INFO]: {}", message).green()),
        MessageType::Warning => println!("{}", format!("[WARNING]: {}", message).yellow()),
    }
}