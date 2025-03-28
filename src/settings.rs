use std::sync::RwLock;

lazy_static::lazy_static! {
    pub static ref GLOBAL_SETTINGS: RwLock<Settings> = RwLock::new(Settings::default());
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub app_name: String,
    pub version: String,
    pub author: String,
    pub max_connections: u32,
    pub enable_logging: bool,
    pub log_file_path: String,
    pub debug_mode: bool,
    pub history_file_path: String,
}

// This is default settings (Change it if you want (e.g. app_name, version, etc))
impl Default for Settings {
    fn default() -> Self {
        Settings {
            app_name: "Seashell".to_string(),
            version: "0.1.0".to_string(),
            author: "Bobrosssrtx".to_string(),
            max_connections: 100,
            enable_logging: true,
            log_file_path: "~/.seashell/seashell.log".to_string(),
            debug_mode: false,
            history_file_path: "~/.seashell/.command_history".to_string(),
        }
    }
}

#[allow(dead_code)]
pub fn get_settings() -> Settings {
    GLOBAL_SETTINGS.read().unwrap().clone()
}

pub fn get_setting(setting: &str) -> Option<String> {
    let settings = GLOBAL_SETTINGS.read().unwrap();
    match setting {
        "app_name" => Some(settings.app_name.clone()),
        "version" => Some(settings.version.clone()),
        "author" => Some(settings.author.clone()),
        "max_connections" => Some(settings.max_connections.to_string()),
        "enable_logging" => Some(settings.enable_logging.to_string()),
        "log_file_path" => Some(settings.log_file_path.clone()),
        "debug_mode" => Some(settings.debug_mode.to_string()),
        "history_file_path" => Some(settings.history_file_path.clone()),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn update_settings(new_settings: Settings) {
    let mut settings = GLOBAL_SETTINGS.write().unwrap();
    *settings = new_settings;
}

#[allow(dead_code)]
pub fn update_setting(setting: &str, value: &str) {
    let mut settings = GLOBAL_SETTINGS.write().unwrap();
    match setting {
        "app_name" => settings.app_name = value.to_string(),
        "version" => settings.version = value.to_string(),
        "author" => settings.author = value.to_string(),
        "max_connections" => settings.max_connections = value.parse().unwrap_or(settings.max_connections),
        "enable_logging" => settings.enable_logging = value.parse().unwrap_or(settings.enable_logging),
        "log_file_path" => settings.log_file_path = value.to_string(),
        "debug_mode" => settings.debug_mode = value.parse().unwrap_or(settings.debug_mode),
        "history_file_path" => settings.history_file_path = value.to_string(),
        _ => {}
    }
}