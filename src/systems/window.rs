use super::System;
use glfw::*;
use std::sync::mpsc::Receiver;

pub struct WindowSystem {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl System for WindowSystem {
    fn initialize() -> Self {
        let glfw = init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(
                800,
                600,
                "DynaSim Interactive Simulation",
                glfw::WindowMode::Windowed,
            )
            .unwrap();
        window.make_current();

        Self {
            glfw,
            window,
            events,
        }
    }

    fn tick(&mut self) -> bool {
        self.window.swap_buffers();
        self.glfw.poll_events();
        for (_, _) in flush_messages(&self.events) {}
        self.window.should_close()
    }
}

impl Drop for WindowSystem {
    fn drop(&mut self) {
        self.window.hide()
    }
}
