use super::prelude::*;
use crate::ecs::resources::{KeyInput, KeyInputQueue, MouseInput, MouseInputQueue};
use crate::impls::platform::prelude::*;
use smallvec::SmallVec;

pub struct PlatformSystem {
    pub win_data: WindowData,
    pub sys_info: SystemInfo,
}

impl SubSystem for PlatformSystem {
    type Args = ();

    fn initialize(cfg: &mut CoreConfig, _: &Self::Args) -> Self {
        let sys_info = get_and_print_system_info();
        let win_data = WindowData::create_window(cfg);

        Self { win_data, sys_info }
    }

    fn prepare(&mut self) {
        self.win_data.window.focus();
        self.win_data.window.show();
    }

    fn tick(&mut self, scenery: &mut Scenery) -> bool {
        self.win_data.context.poll_events();
        for (_, event) in flush_messages(&self.win_data.events) {
            use WindowEvent::*;

            match event {
                Key(key, _, action, modifier) => {
                    let mut key_queue = scenery
                        .resources
                        .get_mut_or_insert(KeyInputQueue(SmallVec::new()));
                    key_queue.0.push(KeyInput {
                        key,
                        action,
                        modifier,
                    });
                }
                MouseButton(button, action, modifier) => {
                    let mut mouse_queue = scenery
                        .resources
                        .get_mut_or_insert(MouseInputQueue(SmallVec::new()));
                    mouse_queue.0.push(MouseInput {
                        button,
                        action,
                        modifier,
                    })
                }
                CursorPos(x, y) => {
                    let mut cursor_pos = scenery
                        .resources
                        .get_mut_or_insert(crate::ecs::resources::CursorPos(0.0, 0.0));
                    cursor_pos.0 = x as f32;
                    cursor_pos.1 = y as f32;
                }
                _ => (),
            }
        }
        !self.win_data.window.should_close()
    }
}
