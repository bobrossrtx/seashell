use crate::{settings::GLOBAL_SETTINGS, utils::printer::Printer};

pub struct Argument {
    pub flag: String,
    pub alias: Option<char>,
    pub help: String,
}

pub struct SeashellArguments {
    args: Vec<String>,
    defined_args: Vec<Argument>,
}

pub struct ParsedArguments {
    pub recognized: Vec<String>,
    pub unrecognized: Vec<String>,
}

impl SeashellArguments {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();

        Self {
            args,
            defined_args: Vec::new(),
        }
    }

    pub fn define_argument(&mut self, flag: &str, alias: Option<char>, help: &str) {
        self.defined_args.push(Argument {
            flag: flag.to_string(),
            alias,
            help: help.to_string(),
        });
    }

    pub fn parser(&self) -> ParsedArguments {
        let mut recognized = Vec::new();
        let mut unrecognized = Vec::new();

        // Always add the first argument (application itself) to recognized
        if let Some(first_arg) = self.args.first() {
            recognized.push(first_arg.clone());
        }

        for arg in self.args.iter().skip(1) {
            if arg.starts_with("--") {
                let flag = &arg[2..];
                if self.defined_args.iter().any(|a| a.flag == flag) {
                    recognized.push(arg.clone());
                } else {
                    unrecognized.push(arg.clone());
                }
            } else if arg.starts_with('-') && arg.len() == 2 {
                let alias = arg.chars().nth(1).unwrap();
                if self.defined_args.iter().any(|a| a.alias == Some(alias)) {
                    recognized.push(arg.clone());
                } else {
                    unrecognized.push(arg.clone());
                }
            } else {
                unrecognized.push(arg.clone());
            }
        }

        ParsedArguments {
            recognized,
            unrecognized,
        }
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn print_help(&self) {
        println!("Usage: seashell [options]\n");
        
        println!("Options:");
        for arg in &self.defined_args {
            if let Some(alias) = arg.alias {
                println!("-{}, --{:<10} {}", alias, arg.flag, arg.help);
            } else {
                println!("    --{:<10} {}", arg.flag, arg.help);
            }
        }
    }

    pub fn print_version(&self) {
        println!("Seashell Version: {}", GLOBAL_SETTINGS.read().unwrap().version);
        println!("Author: {}", GLOBAL_SETTINGS.read().unwrap().author);
        println!("License: MIT");
        println!("Repository: https://github.com/bobrossrtx/seashell");
    }
}

impl ParsedArguments {
    pub fn handle_unrecognized(&self, seashell_args: &SeashellArguments) {
        if !self.unrecognized.is_empty() {
            seashell_args.print_help();
            
            println!("");
            Printer::error("Unrecognized arguments:");
            let mut idx = 1;
            for arg in &self.unrecognized {
                println!("({:02})  |─ {:<10}", idx, arg);
                idx += 1;
            }
            std::process::exit(1);
        }
    }
}