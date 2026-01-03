use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub username: Option<String>,
    pub password: Option<String>,
    pub work_duration_secs: u64,
    pub break_duration_secs: u64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            work_duration_secs: 25 * 60,
            break_duration_secs: 5 * 60,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        confy::load("pomodorotimer", "settings").unwrap_or_default()
    }

    pub fn save(&self) {
        let _ = confy::store("pomodorotimer", "settings", self);
    }

    pub fn get_work_duration(&self) -> Duration {
        Duration::from_secs(self.work_duration_secs)
    }

    pub fn get_break_duration(&self) -> Duration {
        Duration::from_secs(self.break_duration_secs)
    }
}
