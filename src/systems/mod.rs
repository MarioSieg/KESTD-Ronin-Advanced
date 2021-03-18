use super::config::CoreConfig;
use log::info;

pub mod graphics;
pub mod memory;
pub mod platform;

use graphics::GraphicsSystem;
use memory::MemorySystem;
use platform::PlatformSystem;

pub trait System<T> {
    fn initialize(cfg: &mut CoreConfig, data: &T) -> Self;
    fn tick(&mut self) -> bool {
        true
    }
}

pub struct SystemSupervisor {
    pub platform: PlatformSystem,
    pub memory: MemorySystem,
    pub graphics: GraphicsSystem,
}

impl SystemSupervisor {
    pub fn initialize(cfg: &mut CoreConfig) -> Self {
        info!("Initializing platform system...");
        let platform = PlatformSystem::initialize(cfg, &());

        info!("Initializing memory system...");
        let memory = MemorySystem::initialize(cfg, &());

        info!("Initializing graphics system...");
        let graphics = GraphicsSystem::initialize(cfg, &platform.window);

        Self {
            platform,
            memory,
            graphics,
        }
    }

    pub fn tick_all(&mut self) -> bool {
        self.platform.tick() && self.memory.tick() && self.graphics.tick()
    }
}
