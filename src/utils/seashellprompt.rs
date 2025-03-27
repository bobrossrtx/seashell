use crate::utils::environment;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch};
use colored::*;

pub struct SeashellPrompt;

pub fn parse_prompt(template: &str) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    let mut current_styles = vec![]; // Stack to handle multiple styles

    while let Some(c) = chars.next() {
        if c == '/' {
            if let Some(&next_char) = chars.peek() {
                match next_char {
                    'C' => { // Handle color
                        chars.next(); // Consume 'C'
                        if chars.next() == Some('(') {
                            let mut color_name = String::new();
                            while let Some(&next_char) = chars.peek() {
                                if next_char == ')' {
                                    chars.next(); // Consume ')'
                                    break;
                                }
                                color_name.push(next_char);
                                chars.next();
                            }
                            current_styles.push(color_name);
                        }
                    }
                    'B' => { // Handle bold
                        chars.next(); // Consume 'B'
                        current_styles.push("bold".to_string());
                    }
                    'U' => { // Handle underline
                        chars.next(); // Consume 'U'
                        current_styles.push("underline".to_string());
                    }
                    'R' => { // Reset all styles
                        chars.next(); // Consume 'R'
                        current_styles.clear();
                    }
                    _ => {}
                }
                continue;
            }
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

            if let Ok(value) = environment::get_var_as_string(&var_name) {
                let mut styled_value = value.normal();
                for style in &current_styles {
                    match style.as_str() {
                        "bold" => styled_value = styled_value.bold(),
                        "underline" => styled_value = styled_value.underline(),
                        color => styled_value = styled_value.color(color),
                    }
                }
                result.push_str(&styled_value.to_string());
            } else {
                result.push_str(&format!("${{{}}}", var_name)); // Keep as is if not found
            }
            continue;
        }

        let mut styled_char = c.to_string();
        for style in &current_styles {
            match style.as_str() {
                "bold" => styled_char = styled_char.bold().to_string(),
                "underline" => styled_char = styled_char.underline().to_string(),
                color => styled_char = styled_char.color(color).to_string(),
            }
        }
        result.push_str(&styled_char.to_string());
    }

    result
}

impl Prompt for SeashellPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        let prompt_template = environment::get_var_as_string("PROMPT").unwrap_or_else(|_| "[{USER}@{HOSTNAME}]-[{PWD}] $".to_string());
        let formatted_prompt = parse_prompt(&prompt_template);
        formatted_prompt.into()
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        "".into()
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