pub use super::config::CoreConfig;

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
        Self {
            platform: PlatformSystem::initialize(cfg),
            memory: MemorySystem::initialize(cfg),
        }
    }

    pub fn tick_all(&mut self) -> bool {
        self.platform.tick() && self.memory.tick()
    }
}
