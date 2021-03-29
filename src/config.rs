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
    pub default_resource_cache_capacity: usize,
    pub disable_service_routine: bool,
    pub service_routine_minute_interval: u8,
}

impl AppConfig {
    pub const FILE_NAME: &'static str = "app.ini";
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            product_name: "Untitled Product".to_string(),
            product_company: "Default Company".to_string(),
            product_copyright: "Default Copyright".to_string(),
            product_description: "Default Description".to_string(),
            safe_mode: false,
            power_safe_mode: false,
            default_resource_cache_capacity: 128,
            disable_service_routine: false,
            service_routine_minute_interval: 30,
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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
            window_mode: WindowMode::Windowed,
            resolution: (1920, 1080),
            vsync: false,
            gamma_offset: 1.0,
            fps_limit: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum MsaaMode {
    Off = 1,
    X2 = 2,
    X4 = 4,
    X8 = 8,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum GraphicsApi {
    Auto,
    Direct3D11,
    Direct3D12,
    OpenGl,
    Vulkan,
    WebGpu,
}

#[derive(Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub msaa_mode: MsaaMode,
    pub backend_api: GraphicsApi,
    pub max_bind_groups: u32,
    pub max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    pub max_dynamic_storage_buffers_per_pipeline_layout: u32,
    pub max_sampled_textures_per_shader_stage: u32,
    pub max_samplers_per_shader_stage: u32,
    pub max_storage_buffers_per_shader_stage: u32,
    pub max_storage_textures_per_shader_stage: u32,
    pub max_uniform_buffers_per_shader_stage: u32,
    pub max_uniform_buffer_binding_size: u32,
    pub max_push_constant_pool_byte_size: u32,
}

impl GraphicsConfig {
    pub const FILE_NAME: &'static str = "graphics.ini";
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            msaa_mode: MsaaMode::X8,
            backend_api: GraphicsApi::Vulkan,
            max_bind_groups: 4,
            max_dynamic_uniform_buffers_per_pipeline_layout: 8,
            max_dynamic_storage_buffers_per_pipeline_layout: 4,
            max_sampled_textures_per_shader_stage: 16,
            max_samplers_per_shader_stage: 16,
            max_storage_buffers_per_shader_stage: 4,
            max_storage_textures_per_shader_stage: 4,
            max_uniform_buffers_per_shader_stage: 12,
            max_uniform_buffer_binding_size: 16384,
            max_push_constant_pool_byte_size: 256,
        }
    }
}

#[derive(Default)]
pub struct CoreConfig {
    pub application_config: AppConfig,
    pub memory_config: MemoryConfig,
    pub display_config: DisplayConfig,
    pub graphics_config: GraphicsConfig,
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
        let graphics_config = deserialize_config!(config_dir, GraphicsConfig);
        Self {
            application_config,
            memory_config,
            display_config,
            graphics_config,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let config_dir = &PathBuf::from(CONFIG_DIR);
        if !config_dir.exists() {
            fs::create_dir(config_dir)?;
        }
        serialize_config!(config_dir, self, application_config, AppConfig)?;
        serialize_config!(config_dir, self, memory_config, MemoryConfig)?;
        serialize_config!(config_dir, self, display_config, DisplayConfig)?;
        serialize_config!(config_dir, self, graphics_config, GraphicsConfig)
    }
}
