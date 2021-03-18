use super::config::CoreConfig;
use super::systems::SystemSupervisor;
use indicatif::HumanDuration;
use log::*;
use std::io::Write;
use std::process;
use std::time::Instant;

pub struct Engine {
    pub config: CoreConfig,
    pub systems: SystemSupervisor,
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

    fn create_logger() -> Result<(), log::SetLoggerError> {
        use colors::*;
        use fern::*;
        let mut colors = ColoredLevelConfig::new().info(Color::Green);
        colors.warn = Color::Magenta;
        colors.info = Color::BrightBlue;

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
            .chain(std::io::stdout())
            .chain(fern::log_file("engine.log").expect("Failed to create log file!"));

        #[cfg(not(debug_assertions))]
        {
            dispatch = dispatch.level_for("gfx_backend_vulkan", LevelFilter::Off);
            dispatch = dispatch.level_for("gfx_backend_dx11", LevelFilter::Off);
            dispatch = dispatch.level_for("gfx_backend_dx12", LevelFilter::Off);
        }

        dispatch.apply()
    }
}

impl Engine {
    pub fn initialize() -> Box<Self> {
        let clock = Instant::now();
        let _ = Self::create_logger();
        info!("Initializing KESTD Ronin simulation system...");
        info!("PID: {}", process::id());
        let mut config = CoreConfig::load();
        let systems = SystemSupervisor::initialize(&mut config);
        let this = Self { config, systems };
        info!(
            "System online! Boot time: {}",
            HumanDuration(clock.elapsed())
        );
        Box::new(this)
    }

    pub fn run(&mut self) -> u32 {
        info!("Executing simulation...");
        let _ = std::io::stdout().flush();
        let clock = Instant::now();

        let mut cycles = 0;
        while !self.tick() {
            cycles += 1;
        }

        info!(
            "Simulation stopped. Simulated for {} with {} cycles!",
            HumanDuration(clock.elapsed()),
            cycles
        );
        cycles
    }

    pub fn shutdown(&mut self) {}

    fn tick(&mut self) -> bool {
        self.systems.tick_all()
    }
}
