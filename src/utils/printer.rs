use colored::*;

pub struct Printer;

impl Printer {
    pub fn error(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "ERROR".bold().bright_red(), message).red()
        );
    }

    pub fn debug(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "DEBUG".bold().bright_cyan(), message).blue()
        );
    }

    pub fn info(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "INFO".bold().bright_blue(), message).green()
        );
    }

    pub fn warning(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "WARNING".bold().bright_yellow(), message).yellow()
        );
    }

    
    pub fn success(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "SUCCESS".bold().bright_green(), message).green()
        );
    }
}