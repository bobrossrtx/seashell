use std::fs::{OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::env;
use chrono;

lazy_static::lazy_static! {
    static ref LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
}

// Message types
#[derive(Debug)]
pub enum MessageType {
    Error,
    Debug,
    Info,
    Warning,
}

pub fn initialize_logger() -> Result<(), String> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let log_dir = format!("{}/.seashell", home_dir);
    std::fs::create_dir_all(&log_dir).map_err(|e| format!("Failed to create log directory: {}", e))?;

    let log_file_path = format!("{}/seashell.log", log_dir);
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|e| format!("Failed to initialize logger: {}", e))?;

    let mut log_file_lock = LOG_FILE.lock().map_err(|_| "Failed to acquire lock for logger".to_string())?;
    *log_file_lock = Some(log_file);
    Ok(())
}

pub fn log_message(message_type: MessageType, message: &str) -> Result<(), String> {
    let mut log_file_lock = LOG_FILE.lock().map_err(|_| "Failed to acquire lock for logger".to_string())?;
    if let Some(ref mut log_file) = *log_file_lock {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}][{:?}] {}\n", timestamp, message_type, message);
        log_file.write_all(log_entry.as_bytes()).map_err(|e| format!("Failed to write to log file: {}", e))?;
    } else {
        return Err("Logger not initialized".to_string());
    }
    Ok(())
}