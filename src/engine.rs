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
    pub fn initialize() -> Box<Self> {
        let clock = Instant::now();
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
