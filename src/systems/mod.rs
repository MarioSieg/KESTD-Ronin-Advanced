pub use super::config::CoreConfig;
use log::info;

pub mod memory;
pub mod platform;

use memory::MemorySystem;
use platform::PlatformSystem;

pub trait System {
    fn initialize(cfg: &CoreConfig) -> Self;
    fn tick(&mut self) -> bool {
        true
    }
}

pub struct SystemSupervisor {
    pub platform: PlatformSystem,
    pub memory: MemorySystem,
}

impl SystemSupervisor {
    pub fn initialize(cfg: &CoreConfig) -> Self {
        info!("Initializing platform system...");
        let platform = PlatformSystem::initialize(cfg);

        info!("Initializing memory system...");
        let memory = MemorySystem::initialize(cfg);

        Self { platform, memory }
    }

    pub fn tick_all(&mut self) -> bool {
        self.platform.tick() && self.memory.tick()
    }
}
