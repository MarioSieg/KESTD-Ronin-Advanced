use super::config::CoreConfig;
use log::info;

pub mod graphics;
pub mod memory;
pub mod platform;

use crate::ecs::World;
use graphics::GraphicsSystem;
use memory::MemorySystem;
use platform::PlatformSystem;

pub trait SubSystem {
    type Args;

    fn initialize(cfg: &mut CoreConfig, data: &Self::Args) -> Self;
    fn prepare(&mut self) {}
    fn tick(&mut self, _world: &mut World) -> bool {
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

    pub fn prepare_all(&mut self) {
        self.platform.prepare();
        self.memory.prepare();
        self.graphics.prepare();
    }

    pub fn tick_all(&mut self, world: &mut World) -> bool {
        self.platform.tick(world) && self.memory.tick(world) && self.graphics.tick(world)
    }
}

pub mod prelude {
    pub use super::SubSystem;
    pub use crate::config::*;
    pub use crate::ecs::World;
}
