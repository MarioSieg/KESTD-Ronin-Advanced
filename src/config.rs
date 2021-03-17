use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub product_name: String,
    pub product_company: String,
    pub product_copyright: String,
    pub product_description: String,
    pub safe_mode: bool,
    pub power_safe_mode: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CoreConfig {
    pub app_config: AppConfig,
}

impl CoreConfig {
    pub const FILE_NAME: &'static str = "engine.ini";

    pub fn load() -> Self {
        if !Path::new(Self::FILE_NAME).exists() {
            let this = Self::default();
            let _ = this.save();
            this
        } else {
            serde_yaml::from_str(
                &fs::read_to_string(Path::new(Self::FILE_NAME)).unwrap_or_default(),
            )
            .unwrap_or_default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        fs::write(
            Path::new(Self::FILE_NAME),
            serde_yaml::to_string(self).unwrap_or_default(),
        )
    }
}
