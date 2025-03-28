use colored::*;

pub struct Printer;

#[allow(dead_code)]
impl Printer {
    pub fn error(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "ERROR".bold().bright_red(), message).white()
        );
    }

    pub fn debug(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "DEBUG".bold().bright_cyan(), message).white()
        );
    }

    pub fn info(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "INFO".bold().bright_blue(), message).white()
        );
    }

    pub fn warning(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "WARNING".bold().bright_yellow(), message).white()
        );
    }

    
    pub fn success(message: &str) {
        println!(
            "{}",
            format!("[{}]: {}", "SUCCESS".bold().bright_green(), message).green()
        );
    }
}