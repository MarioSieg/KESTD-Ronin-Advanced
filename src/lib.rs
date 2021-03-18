pub mod config;
pub mod engine;
pub mod systems;

pub fn setup_logger() -> simple_logger::SimpleLogger {
    #[cfg(debug_assertions)]
    {
        simple_logger::SimpleLogger::new()
    }

    #[cfg(not(debug_assertions))]
    {
        let mut logger = simple_logger::SimpleLogger::new();
        logger = logger.with_module_level("gfx_backend_vulkan", log::LevelFilter::Off);
        logger = logger.with_module_level("gfx_backend_dx11", log::LevelFilter::Off);
        logger = logger.with_module_level("gfx_backend_dx12", log::LevelFilter::Off);
        logger
    }
}
