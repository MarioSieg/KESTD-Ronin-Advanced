pub mod window;

pub trait System {
    fn initialize() -> Self;
    fn tick(&mut self) -> bool;
}

pub struct SystemSupervisor {
    pub window: window::WindowSystem,
}

impl SystemSupervisor {
    pub fn initialize() -> Self {
        Self {
            window: window::WindowSystem::initialize(),
        }
    }

    pub fn tick_all(&mut self) -> bool {
        self.window.tick()
    }
}
