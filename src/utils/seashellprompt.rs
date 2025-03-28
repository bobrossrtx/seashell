use crate::utils::environment;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch};
use ansi_term::Colour;

pub struct SeashellPrompt;

pub fn parse_prompt(template: &str) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut color_name = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char == '}' {
                    chars.next(); // Consume '}'
                    break;
                }
                color_name.push(next_char);
                chars.next();
            }

            match color_name.as_str() {
                "red" => result.push_str(&Colour::Red.prefix().to_string()),
                "green" => result.push_str(&Colour::Green.prefix().to_string()),
                "yellow" => result.push_str(&Colour::Yellow.prefix().to_string()),
                "blue" => result.push_str(&Colour::Blue.prefix().to_string()),
                "magenta" => result.push_str(&Colour::Purple.prefix().to_string()),
                "cyan" => result.push_str(&Colour::Cyan.prefix().to_string()),
                "white" => result.push_str(&Colour::White.prefix().to_string()),
                "reset" => result.push_str(&Colour::White.suffix().to_string()),
                _ => result.push_str(&format!("{{{}}}", color_name)), // Keep as is if unknown
            }
            continue;
        } else if c == '$' && chars.peek() == Some(&'{') {
            chars.next(); // Consume '{'
            let mut var_name = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char == '}' {
                    chars.next(); // Consume '}'
                    break;
                }
                var_name.push(next_char);
                chars.next();
            }

            if let Ok(mut value) = environment::get_var_as_string(&var_name) {
                if var_name == "PWD" {
                    if let Ok(home) = environment::get_var_as_string("HOME") {
                        if value.starts_with(&home) {
                            value = value.replacen(&home, "~", 1);
                        }
                    }
                }
                result.push_str(&value);
            } else {
                result.push_str(&format!("${{{}}}", var_name)); // Keep as is if not found
            }
            continue;
        } else if c == '\\' {
            if let Some(&escaped_char) = chars.peek() {
                result.push(escaped_char);
                chars.next(); // Consume escaped character
            }
            continue;
        }

        result.push(c);
    }

    result
}

impl Prompt for SeashellPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        let prompt_template = environment::get_var_as_string("PROMPT_LEFT").unwrap_or_else(|_| "[{USER}@{HOSTNAME}]-[{PWD}] $".to_string());
        let formatted_prompt = parse_prompt(&prompt_template);
        formatted_prompt.into()
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        let prompt_template = environment::get_var_as_string("PROMPT_RIGHT").unwrap_or_else(|_| " ".to_string());
        let formatted_prompt = parse_prompt(&prompt_template);
        formatted_prompt.into()
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> std::borrow::Cow<str> {
        "".into()
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        "".into()
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        "".into()
    }
}