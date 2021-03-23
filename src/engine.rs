use super::config::CoreConfig;
use super::ecs::{self, World};
use super::systems::SystemSupervisor;
use humantime::Duration;
use log::*;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::Instant;

pub struct Engine {
    pub config: CoreConfig,
    pub systems: SystemSupervisor,
    pub world: World,
}

impl Engine {
    fn log_level_filter() -> log::LevelFilter {
        #[cfg(debug_assertions)]
        {
            log::LevelFilter::Trace
        }
        #[cfg(not(debug_assertions))]
        {
            log::LevelFilter::Info
        }
    }

    const LOGGER_DIR: &'static str = "proto";

    fn create_logger() -> Result<(), log::SetLoggerError> {
        use colors::*;
        use fern::*;
        let mut colors = ColoredLevelConfig::new().info(Color::Green);
        colors.warn = Color::Magenta;
        colors.info = Color::BrightBlue;

        let log_dir = Path::new(Self::LOGGER_DIR);
        if !log_dir.exists() && fs::create_dir(log_dir).is_err() {
            warn!(
                "Failed to create log directory: {:?}! Log file creation might fail too!",
                log_dir
            );
        }

        let log_file_path = String::from(log_dir.to_str().unwrap_or_default())
            + &chrono::Local::now()
                .format("/engine_session_%Y_%m_%d_%H_%M_%S.log")
                .to_string();
        let log_file = fern::log_file(&log_file_path);

        #[allow(unused_mut)]
        let mut dispatch = Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    colors.color(record.level()),
                    message
                ))
            })
            .level(Self::log_level_filter())
            .chain(std::io::stdout());

        if let Ok(file) = log_file {
            dispatch = dispatch.chain(file);
        } else {
            warn!("Failed to create log file: {:?}", log_file_path);
        }

        #[cfg(not(debug_assertions))]
        {
            dispatch = dispatch.level_for("gfx_backend_vulkan", LevelFilter::Off);
            dispatch = dispatch.level_for("gfx_backend_dx11", LevelFilter::Off);
            dispatch = dispatch.level_for("gfx_backend_dx12", LevelFilter::Off);
            dispatch = dispatch.level_for("naga", LevelFilter::Off);
        }

        dispatch.apply()
    }

    fn install_panic_hook() {
        // Only use custom panic handler if we are in release mode:
        #[cfg(not(debug_assertions))]
        std::panic::set_hook(Box::new(|panic_info: &core::panic::PanicInfo| {
            // get info:
            let (file, line) = if let Some(loc) = panic_info.location() {
                (loc.file(), loc.line())
            } else {
                ("", 0)
            };
            let info = panic_info.payload().downcast_ref::<&str>().unwrap_or(&"");

            // print to stdout:
            println!(
                "System panic occurred in file '{}' at line {}! Message: {:?}",
                file, line, info
            );
            let _ = std::io::stdout().flush();

            // create message box:
            let _ = msgbox::create(
                "Engine System Panic",
                &format!(
                    "System panic occurred in file '{}' at line {}! Message: {:?}",
                    file, line, info
                ),
                msgbox::IconType::Error,
            );
        }));
    }

    pub fn initialize() -> Box<Self> {
        Self::install_panic_hook();
        let clock = Instant::now();
        let _ = Self::create_logger();

        info!("Initializing KESTD Ronin simulation system...");
        info!("PID: {}", process::id());
        info!(
            "Working directory: {:?}",
            std::env::current_dir().unwrap_or_default()
        );
        info!(
            "Executable: {:?}",
            std::env::current_exe().unwrap_or_default()
        );

        let mut config = CoreConfig::load();
        let systems = SystemSupervisor::initialize(&mut config);
        let mut world = World::default();
        ecs::initialize_default_world(&systems, &mut world);

        let this = Self {
            config,
            systems,
            world,
        };

        info!(
            "System online! Boot time: {}",
            Duration::from(clock.elapsed())
        );
        Box::new(this)
    }

    pub fn run(&mut self) -> u32 {
        info!("Preparing systems...");
        self.systems.prepare_all();
        info!("Executing simulation...");
        let _ = std::io::stdout().flush();
        let clock = Instant::now();

        let mut cycles = 0;
        while self.tick() {
            cycles += 1;
        }

        info!(
            "Simulation stopped. Simulated for {} with {} cycles!",
            Duration::from(clock.elapsed()),
            cycles
        );
        cycles
    }

    pub fn shutdown(&mut self) {}

    fn tick(&mut self) -> bool {
        self.systems.tick_all(&mut self.world)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;
