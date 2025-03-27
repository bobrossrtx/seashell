use std::collections::HashMap;
use crate::utils::command_metadata::CommandMetadata;

pub struct ArgDefinition {
    pub flag: Option<String>,
    pub alias: Option<String>,
    pub help: String,
}

pub struct ArgParser {
    pub flags: HashMap<String, bool>,
    pub options: HashMap<String, String>,
    pub positional: Vec<String>,
}

impl ArgParser {
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
            options: HashMap::new(),
            positional: Vec::new(),
        }
    }

    pub fn parse(&mut self, args: &[String], definitions: &[ArgDefinition], metadata: &mut CommandMetadata) {
        // Populate flags_and_options metadata in CommandMetadata
        for def in definitions {
            metadata.flags_and_options.push((def.flag.clone(), def.alias.clone(), def.help.clone()));
        }

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                let key_value: Vec<&str> = arg[2..].splitn(2, '=').collect();
                let key = key_value[0];
                let value = key_value.get(1).map(|v| v.to_string());

                if let Some(def) = definitions.iter().find(|d| d.alias.as_deref() == Some(key)) {
                    if let Some(value) = value {
                        self.options.insert(key.to_string(), value);
                    } else {
                        self.flags.insert(key.to_string(), true);
                    }
                }
            } else if arg.starts_with('-') {
                for flag in arg.chars().skip(1) {
                    if let Some(def) = definitions.iter().find(|d| d.flag.as_deref() == Some(&flag.to_string())) {
                        if let Some(alias) = &def.alias {
                            self.flags.insert(alias.clone(), true);
                        } else {
                            self.flags.insert(flag.to_string(), true);
                        }
                    }
                }
            } else {
                self.positional.push(arg.to_string());
            }
        }
    }

    pub fn is_flag_set(&self, key: &str) -> bool {
        *self.flags.get(key).unwrap_or(&false)
    }

    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }
}