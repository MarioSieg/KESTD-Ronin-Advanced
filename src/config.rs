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

impl AppConfig {
    pub const FILE_NAME: &'static str = "config/app.ini";
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

impl MemoryConfig {
    pub const FILE_NAME: &'static str = "config/memory.ini";
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            default_string_pool_size: 16384,
            default_pool_allocator_size: 1024 * 1024 * 512, // 512 MB
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum WindowMode {
    FullScreen,
    Windowed,
}

#[derive(Serialize, Deserialize)]
pub struct DisplayConfig {
    pub window_mode: WindowMode,
    pub vsync: bool,
    pub resolution: (u16, u16),
    pub gamma_offset: f32,
    pub fps_limit: Option<u16>,
}

impl DisplayConfig {
    pub const FILE_NAME: &'static str = "config/display.ini";
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            window_mode: WindowMode::FullScreen,
            resolution: (1920, 1080),
            vsync: false,
            gamma_offset: 1.0,
            fps_limit: None,
        }
    }
}

#[derive(Default)]
pub struct CoreConfig {
    pub application_config: AppConfig,
    pub memory_config: MemoryConfig,
    pub display_config: DisplayConfig,
}

impl CoreConfig {
    pub fn load() -> Self {
        if !Path::new("config/").exists() {
            let this = Self::default();
            let _ = this.save();
            return this;
        }
        let application_config = serde_yaml::from_str::<AppConfig>(
            &fs::read_to_string(Path::new(AppConfig::FILE_NAME)).unwrap_or_default(),
        )
        .unwrap_or_default();
        let memory_config = serde_yaml::from_str::<MemoryConfig>(
            &fs::read_to_string(Path::new(MemoryConfig::FILE_NAME)).unwrap_or_default(),
        )
        .unwrap_or_default();
        let display_config = serde_yaml::from_str::<DisplayConfig>(
            &fs::read_to_string(Path::new(DisplayConfig::FILE_NAME)).unwrap_or_default(),
        )
        .unwrap_or_default();
        Self {
            application_config,
            memory_config,
            display_config,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        if !Path::new("config/").exists() {
            fs::create_dir("config/")?;
        }
        fs::write(
            Path::new(AppConfig::FILE_NAME),
            serde_yaml::to_string(&self.application_config).unwrap_or_default(),
        )?;
        fs::write(
            Path::new(DisplayConfig::FILE_NAME),
            serde_yaml::to_string(&self.display_config).unwrap_or_default(),
        )?;
        fs::write(
            Path::new(MemoryConfig::FILE_NAME),
            serde_yaml::to_string(&self.memory_config).unwrap_or_default(),
        )
    }
}
