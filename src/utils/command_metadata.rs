#[derive(Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub usage: String,
    pub flags_and_options: Vec<(Option<String>, Option<String>, String)>, // (Flag, Alias, Help)
}

impl CommandMetadata {
    pub fn new(name: &str, description: &str, version: &str, usage: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: version.to_string(),
            usage: usage.to_string(),
            flags_and_options: Vec::new(),
        }
    }

    pub fn display_help(&self) {
        println!("{} - {}\nVersion: {}\n\nUsage:\n  {}\n", self.name, self.description, self.version, self.usage);
        if !self.flags_and_options.is_empty() {
            println!("Options and Flags:");
            for (flag, alias, desc) in &self.flags_and_options {
                match (flag, alias) {
                    (Some(f), Some(a)) => println!("  {:<10} {:<10} {}", format!("-{}", f), format!("--{}", a), desc),
                    (Some(f), None) => println!("  {:<10} {:<10} {}", format!("-{}", f), "", desc),
                    (None, Some(a)) => println!("  {:<10} {:<10} {}", "", format!("--{}", a), desc),
                    _ => (),
                }
            }
        }
    }
}