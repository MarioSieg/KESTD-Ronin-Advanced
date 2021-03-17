use super::config::CoreConfig;
use super::systems::SystemSupervisor;
use std::io::Write;
use std::time::Instant;

pub struct Engine {
    pub config: CoreConfig,
    pub systems: SystemSupervisor,
}

impl Engine {
    pub fn initialize() -> Self {
        println!("Initializing KESTD Ronin simulation system...");
        let clock = Instant::now();
        let this = Self {
            config: CoreConfig::load(),
            systems: SystemSupervisor::initialize(),
        };
        println!(
            "System online! Boot time: {}s",
            clock.elapsed().as_secs_f32()
        );
        this
    }

    pub fn run(&mut self) -> u32 {
        println!("Executing simulation...");
        let _ = std::io::stdout().flush();
        let clock = Instant::now();

        let mut cycles = 0;
        while !self.tick() {
            cycles += 1;
        }

        println!(
            "Simulation stopped. Simulated for {}s with {} cycles!",
            clock.elapsed().as_secs_f32(),
            cycles
        );
        cycles
    }

    pub fn shutdown(&mut self) {}

    fn tick(&mut self) -> bool {
        self.systems.tick_all()
    }
}

impl std::default::Default for Engine {
    fn default() -> Self {
        Self::initialize()
    }
}
