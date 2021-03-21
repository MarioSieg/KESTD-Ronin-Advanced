use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_DIR: &str = "config";

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
    pub const FILE_NAME: &'static str = "app.ini";
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
    pub default_memory_pool_size: usize,
}

impl MemoryConfig {
    pub const FILE_NAME: &'static str = "memory.ini";
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            default_string_pool_size: 16384,
            default_memory_pool_size: 1024 * 1024 * 512, // 512 MB
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
    pub const FILE_NAME: &'static str = "display.ini";
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

macro_rules! deserialize_config {
    ($dir:ident, $type:ty) => {
    serde_yaml::from_str::<$type>(
            &fs::read_to_string($dir.join(Path::new(<$type>::FILE_NAME)))
                .unwrap_or_default(),
        )
        .unwrap_or_else(|_| {
            warn!(
                "Failed to load: \"{}\"! Setting default values...",
                <$type>::FILE_NAME
            );
            warn!("Trying autofix by removing config files! Clean config files will be recreated when rebooting the engine!");
            let _ = fs::remove_dir_all(&$dir);
            std::default::Default::default()
        })
    };
}

macro_rules! serialize_config {
    ($dir:ident, $self:ident, $data:ident, $type:ty) => {
        fs::write(
            $dir.join(Path::new(<$type>::FILE_NAME)),
            serde_yaml::to_string(&$self.$data).unwrap_or_default(),
        )
    };
}

impl CoreConfig {
    pub fn load() -> Self {
        let config_dir = PathBuf::from(CONFIG_DIR);
        info!(
            "Parsing config from dir: {:?}",
            fs::canonicalize(&config_dir).unwrap_or_default()
        );
        if !config_dir.exists() {
            warn!("Config directory does not exist! Creating config...");
            let this = Self::default();
            let _ = this.save();
            return this;
        }
        let application_config = deserialize_config!(config_dir, AppConfig);
        let memory_config = deserialize_config!(config_dir, MemoryConfig);
        let display_config = deserialize_config!(config_dir, DisplayConfig);
        Self {
            application_config,
            memory_config,
            display_config,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let config_dir = &PathBuf::from(CONFIG_DIR);
        if !config_dir.exists() {
            fs::create_dir(config_dir)?;
        }
        serialize_config!(config_dir, self, application_config, AppConfig)?;
        serialize_config!(config_dir, self, memory_config, MemoryConfig)?;
        serialize_config!(config_dir, self, display_config, DisplayConfig)
    }
}
