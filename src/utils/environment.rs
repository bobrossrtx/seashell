use std::collections::HashMap;
use std::sync::Mutex;
use gethostname;

use super::logger;

lazy_static::lazy_static! {
    pub static ref ENV_VARS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn initialize_default_vars() -> HashMap<String, String> {
    if let Ok(_) = std::env::var("USER") {
        let mut env_vars = ENV_VARS.lock().expect("Failed to lock ENV_VARS");
        if let Ok(user) = std::env::var("USER") {
            env_vars.insert("USER".to_string(), user);
        }
        if let Ok(shell) = std::env::var("SHELL") {
            env_vars.insert("SHELL".to_string(), shell);
        }
        if let Ok(home) = std::env::var("HOME") {
            env_vars.insert("HOME".to_string(), home);
        }
        if let Ok(path) = std::env::var("PATH") {
            env_vars.insert("PATH".to_string(), path);
        }
        if let Ok(term) = std::env::var("TERM") {
            env_vars.insert("TERM".to_string(), term);
        }
        if let Ok(lang) = std::env::var("LANG") {
            env_vars.insert("LANG".to_string(), lang);
        }
        if let Ok(editor) = std::env::var("EDITOR") {
            env_vars.insert("EDITOR".to_string(), editor);
        }

        let hostname = gethostname::gethostname()
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        env_vars.insert("HOSTNAME".to_string(), hostname);
        env_vars.insert("PWD".to_string(), std::env::current_dir().unwrap_or_else(|_| "/".into()).to_str().unwrap_or("/").to_string());
        // env_vars.insert("PROMPT_LEFT".to_string(), "/C(red)┌──[/C(white)${USER}/C(bright_yellow)@/C(cyan)${HOSTNAME}/C(red)]/C(red)─/C(red)[/C(green)${PWD}/C(red)]\n└──╼".to_string());
        env_vars.insert("PROMPT_LEFT".to_string(), "{red}┌──[{white}${USER}{yellow}@{cyan}${HOSTNAME}{red}]{red}─[{green}${PWD}{red}]\r\n{red}└──╼ {yellow}$ ".to_string());
        // env_vars.insert("PROMPT_RIGHT".to_string(), ("{}", ).to_string());
    }

    // Log shell session's default vars
    logger::log_message(logger::MessageType::Info, "Shell session's default vars initialized").ok();
    for (key, value) in ENV_VARS.lock().expect("Failed to lock ENV_VARS").iter() {
        logger::log_message(logger::MessageType::Info, &format!("{}={}", key, value)).ok();
    }

    ENV_VARS.lock().expect("Failed to lock ENV_VARS").clone()
}

pub fn set_var(key: &str, value: &str) -> Result<(), String> {
    let mut env_vars = ENV_VARS.lock().map_err(|_| "set_var: failed to acquire lock".to_string())?;
    env_vars.insert(key.to_string(), value.to_string());
    Ok(())
}

pub fn get_var(key: &str) -> Result<String, String> {
    let env_vars = ENV_VARS.lock().map_err(|_| "get_var: failed to acquire lock".to_string())?;
    env_vars.get(key).cloned().ok_or_else(|| format!("get_var: '{}' not found", key))
}

pub fn unset_var(key: &str) -> Result<(), String> {
    let mut env_vars = ENV_VARS.lock().map_err(|_| "unset_var: failed to acquire lock".to_string())?;
    if env_vars.remove(key).is_some() {
        Ok(())
    } else {
        Err(format!("unset_var: '{}' not found", key))
    }
}

pub fn list_vars() -> Result<Vec<(String, String)>, String> {
    let env_vars = ENV_VARS.lock().map_err(|_| "list_vars: failed to acquire lock".to_string())?;
    Ok(env_vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
}

pub fn change_directory(path: &str) -> Result<(), String> {
    std::env::set_current_dir(path).map_err(|e| format!("cd: {}", e))?;
    set_var("PWD", path)?;
    Ok(())
}

pub fn get_var_as_string(key: &str) -> Result<String, String> {
    let env_vars = ENV_VARS.lock().map_err(|_| "get_var_as_string: failed to acquire lock".to_string())?;
    env_vars.get(key).cloned().ok_or_else(|| format!("get_var_as_string: '{}' not found", key))
}