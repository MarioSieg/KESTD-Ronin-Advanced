use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub product_name: String,
    pub product_company: String,
    pub product_copyright: String,
    pub product_description: String,
    pub safe_mode: bool,
    pub power_safe_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            product_name: String::from("Untitled Product"),
            product_company: String::from("Default Company"),
            product_copyright: String::from("Default Copyright"),
            product_description: String::from("Default Description"),
            safe_mode: false,
            power_safe_mode: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemoryConfig {
    pub default_string_pool_size: usize,
    pub default_pool_allocator_size: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            default_string_pool_size: 16384,
            default_pool_allocator_size: 1024 * 1024 * 512, // 512 MB
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct CoreConfig {
    pub app_config: AppConfig,
    pub mem_config: MemoryConfig,
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
