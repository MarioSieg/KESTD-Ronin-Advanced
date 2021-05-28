use super::config::CoreConfig;
use super::ecs::{self, Scenery};
use super::resources::ResourceManager;
use super::service;
use super::systems::SystemSupervisor;
use clokwerk::{Interval, ScheduleHandle, Scheduler};
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
    pub scenery: Scenery,
    pub resource_manager: ResourceManager,
    pub service_scheduler_thread: Option<ScheduleHandle>,
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

        println!("{}", LOGO);
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
        let mut scenery = Scenery::default();
        let mut resource_manager = ResourceManager::with_capacity(
            config.application_config.default_resource_cache_capacity,
        );

        let service_scheduler_thread = if !config.application_config.disable_service_routine {
            let interval = config
                .application_config
                .service_routine_minute_interval
                .clamp(1, 60) as u64;
            info!(
                "Starting service routine thread scheduler with interval of {} Minutes",
                interval
            );
            let mut service_scheduler = Scheduler::new();
            service_scheduler
                .every(Interval::Minutes(interval as u32))
                .run(service::service_routine);
            Some(service_scheduler.watch_thread(std::time::Duration::new(interval, 0)))
        } else {
            warn!("Service routine is disabled! This is not recommended and might lead to system instability!");
            None
        };

        info!("Initializing scenery...");
        let scenery_clock = Instant::now();
        ecs::initialize_default_scenery(&systems, &mut scenery, &mut resource_manager);
        info!(
            "Scenery is initialized! Time: {}",
            Duration::from(scenery_clock.elapsed())
        );

        let this = Self {
            config,
            systems,
            scenery,
            resource_manager,
            service_scheduler_thread,
        };

        info!(
            "System online! Boot time: {}",
            Duration::from(clock.elapsed())
        );
        Box::new(this)
    }

    pub fn run(&mut self) -> u32 {
        info!("Preparing systems...");
        self.systems.prepare_all(&mut self.scenery);
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

    pub fn shutdown(&mut self) {
        println!("Shutting down simulation system...");
    }

    fn tick(&mut self) -> bool {
        self.systems.tick_all(&mut self.scenery)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

const LOGO: &str = r#"
KKKKKKKKK    KKKKKKKEEEEEEEEEEEEEEEEEEEEEE   SSSSSSSSSSSSSSS TTTTTTTTTTTTTTTTTTTTTTTDDDDDDDDDDDDD
K:::::::K    K:::::KE::::::::::::::::::::E SS:::::::::::::::ST:::::::::::::::::::::TD::::::::::::DDD
K:::::::K    K:::::KE::::::::::::::::::::ES:::::SSSSSS::::::ST:::::::::::::::::::::TD:::::::::::::::DD
K:::::::K   K::::::KEE::::::EEEEEEEEE::::ES:::::S     SSSSSSST:::::TT:::::::TT:::::TDDD:::::DDDDD:::::D
KK::::::K  K:::::KKK  E:::::E       EEEEEES:::::S            TTTTTT  T:::::T  TTTTTT  D:::::D    D:::::D
  K:::::K K:::::K     E:::::E             S:::::S                    T:::::T          D:::::D     D:::::D
  K::::::K:::::K      E::::::EEEEEEEEEE    S::::SSSS                 T:::::T          D:::::D     D:::::D
  K:::::::::::K       E:::::::::::::::E     SS::::::SSSSS            T:::::T          D:::::D     D:::::D
  K:::::::::::K       E:::::::::::::::E       SSS::::::::SS          T:::::T          D:::::D     D:::::D
  K::::::K:::::K      E::::::EEEEEEEEEE          SSSSSS::::S         T:::::T          D:::::D     D:::::D
  K:::::K K:::::K     E:::::E                         S:::::S        T:::::T          D:::::D     D:::::D
KK::::::K  K:::::KKK  E:::::E       EEEEEE            S:::::S        T:::::T          D:::::D    D:::::D
K:::::::K   K::::::KEE::::::EEEEEEEE:::::ESSSSSSS     S:::::S      TT:::::::TT      DDD:::::DDDDD:::::D
K:::::::K    K:::::KE::::::::::::::::::::ES::::::SSSSSS:::::S      T:::::::::T      D:::::::::::::::DD
K:::::::K    K:::::KE::::::::::::::::::::ES:::::::::::::::SS       T:::::::::T      D::::::::::::DDD
KKKKKKKKK    KKKKKKKEEEEEEEEEEEEEEEEEEEEEE SSSSSSSSSSSSSSS         TTTTTTTTTTT      DDDDDDDDDDDDD
"#;
