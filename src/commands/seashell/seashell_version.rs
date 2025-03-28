use std::process::Command;
use crate::utils::{arg_parser::{ArgDefinition, ArgParser}, command_metadata::CommandMetadata};

pub struct SeashellVersionCommand {
    pub metadata: CommandMetadata,
}

impl SeashellVersionCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "seashell_version",
                "Display Seashell version and dependencies",
                "1.0.0",
                "seashell_version",
            ),
        }
    }

    pub fn execute(&mut self, args: &[String]) -> Result<(), String> {
        let definitions = [
            ArgDefinition {
                flag: Some("h".to_string()),
                alias: Some("help".to_string()),
                help: "Display this help message".to_string(),
            }
        ];

        let mut parser = ArgParser::new();
        parser.parse(args, &definitions, &mut self.metadata);

        if parser.is_flag_set("help") {
            self.metadata.display_help();
            return Ok(());
        }
        
        // Print Seashell version
        println!("Seashell Version: 1.0.0");

        // Print Rust version
        if let Ok(output) = Command::new("rustc").arg("--version").output() {
            if output.status.success() {
                let rust_version = String::from_utf8_lossy(&output.stdout);
                println!("Rust Version: {}", rust_version.trim());
            } else {
                eprintln!("Failed to retrieve Rust version.");
            }
        } else {
            eprintln!("Rust is not installed or not in PATH.");
        }

        // Print Cargo dependencies
        println!("Seashell Dependencies:");
        if let Ok(output) = Command::new("cargo").args(&["tree", "--depth", "1"]).output() {
            if output.status.success() {
            let dependencies = String::from_utf8_lossy(&output.stdout);
            println!("{}", dependencies);
            } else {
            eprintln!("Failed to retrieve top-level dependencies. Make sure Cargo is installed.");
            }
        } else {
            eprintln!("Cargo is not installed or not in PATH.");
        }

        Ok(())
    }
}