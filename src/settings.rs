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

pub fn get_settings() -> Settings {
    GLOBAL_SETTINGS.read().unwrap().clone()
}

pub fn update_settings(new_settings: Settings) {
    let mut settings = GLOBAL_SETTINGS.write().unwrap();
    *settings = new_settings;
}