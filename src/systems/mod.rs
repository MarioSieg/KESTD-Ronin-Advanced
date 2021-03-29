use super::config::CoreConfig;
use log::info;

pub mod graphics;
pub mod memory;
pub mod platform;

use crate::ecs::Scenery;
use graphics::GraphicsSystem;
use memory::MemorySystem;
use platform::PlatformSystem;

pub trait SubSystem {
    type Args;

    fn initialize(cfg: &mut CoreConfig, data: &Self::Args) -> Self;
    fn prepare(&mut self, _scenery: &mut Scenery) {}
    fn tick(&mut self, _scenery: &mut Scenery) -> bool {
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
        let graphics = GraphicsSystem::initialize(cfg, &platform.win_data.window);

        Self {
            platform,
            memory,
            graphics,
        }
    }

    pub fn prepare_all(&mut self, scenery: &mut Scenery) {
        self.platform.prepare(scenery);
        self.memory.prepare(scenery);
        self.graphics.prepare(scenery);
    }

    pub fn tick_all(&mut self, scenery: &mut Scenery) -> bool {
        self.platform.tick(scenery) && self.memory.tick(scenery) && self.graphics.tick(scenery)
    }
}

pub mod prelude {
    pub use super::SubSystem;
    pub use crate::config::*;
    pub use crate::ecs::Scenery;
}
