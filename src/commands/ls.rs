use std::fs;
use std::os::unix::fs::MetadataExt;
use std::time::UNIX_EPOCH;
use std::os::unix::fs::PermissionsExt;
use crate::utils::arg_parser::{ArgDefinition, ArgParser};
use crate::utils::command_metadata::CommandMetadata;

pub struct LsCommand {
    pub metadata: CommandMetadata,
}

impl LsCommand {
    pub fn new() -> Self {
        Self {
            metadata: CommandMetadata::new(
                "ls",
                "List directory contents",
                "1.0.0",
                "ls [OPTIONS] [PATH]",
            ),
        }
    }

    pub fn execute(&mut self, args: &[String]) -> Result<(), String> {
        let definitions = [
            ArgDefinition {
                flag: Some("h".to_string()),
                alias: Some("help".to_string()),
                help: "Display this help message".to_string(),
            },
            ArgDefinition {
                flag: Some("a".to_string()),
                alias: Some("all".to_string()),
                help: "Show hidden files".to_string(),
            },
            ArgDefinition {
                flag: Some("l".to_string()),
                alias: None,
                help: "Use a long listing format".to_string(),
            },
            ArgDefinition {
                flag: Some("R".to_string()),
                alias: None,
                help: "List subdirectories recursively".to_string(),
            },
        ];

        let mut parser = ArgParser::new();
        parser.parse(args, &definitions, &mut self.metadata);

        if parser.is_flag_set("help") {
            self.metadata.display_help();
            return Ok(());
        }

        let default_path = ".".to_string();
        let path = parser.positional.get(0).unwrap_or(&default_path);
        let show_hidden = parser.is_flag_set("a");
        let detailed = parser.is_flag_set("l");
        let recursive = parser.is_flag_set("R");

        list_directory(path, show_hidden, detailed, recursive)?;
        Ok(())
    }
}

fn list_directory(path: &str, show_hidden: bool, detailed: bool, recursive: bool) -> Result<(), String> {
    let entries = fs::read_dir(path).map_err(|e| format!("ls: failed to read directory '{}': {}", path, e))?;
    let mut entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| show_hidden || !entry.file_name().to_string_lossy().starts_with('.'))
        .collect();

    // Sort entries alphabetically
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in &entries {
        if detailed {
            let metadata = entry.metadata().map_err(|e| format!("ls: failed to get metadata: {}", e))?;
            let file_type = if metadata.is_dir() { 'd' } else { '-' };
            let permissions = metadata.permissions();
            let size = metadata.size();
            let modified = metadata.modified().ok().and_then(|m| m.duration_since(UNIX_EPOCH).ok());
            let modified_time = modified.map(|m| m.as_secs()).unwrap_or(0);

            println!("{} {:o} {:>8} {:>10} {}", file_type, permissions.mode(), size, modified_time, entry.file_name().to_string_lossy());
        } else {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }

    if recursive {
        for entry in &entries {
            let metadata = entry.metadata().map_err(|e| format!("ls: failed to get metadata: {}", e))?;
            if metadata.is_dir() {
                println!("\n{}:", entry.path().display());
                list_directory(&entry.path().to_string_lossy(), show_hidden, detailed, recursive)?;
            }
        }
    }

    Ok(())
}