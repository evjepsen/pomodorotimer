use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_user: Option<String>,
}

impl Config {
    /// Get the config file path in the standard location for Linux
    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        let app_config_dir = config_dir.join("pomodorotimer");
        
        // Create the config directory if it doesn't exist
        fs::create_dir_all(&app_config_dir)?;
        
        Ok(app_config_dir.join("config.toml"))
    }
    
    /// Load configuration from file, or create default if it doesn't exist
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Config::default())
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    /// Set the default user and save to config
    pub fn set_default_user(&mut self, username: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.default_user = username;
        self.save()
    }
}
