use super::prelude::*;
use crate::ecs::resources::{CursorPos, KeyInputStateCollection, MouseInputStateCollection};
use crate::impls::platform::prelude::*;

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

    fn prepare(&mut self, scenery: &mut Scenery) {
        scenery.resources.insert(KeyInputStateCollection::default());
        scenery
            .resources
            .insert(MouseInputStateCollection::default());
        scenery.resources.insert(CursorPos(0.0, 0.0));
        self.win_data.window.focus();
        self.win_data.window.show();
    }

    fn tick(&mut self, scenery: &mut Scenery) -> bool {
        self.win_data.context.poll_events();
        for (_, event) in flush_messages(&self.win_data.events) {
            use WindowEvent::*;

            match event {
                Key(key, _, action, _) => {
                    let mut key_queue = scenery
                        .resources
                        .get_mut::<KeyInputStateCollection>()
                        .unwrap();
                    if action == Action::Press {
                        key_queue.push(key);
                    } else if action == Action::Release {
                        key_queue.pop(key);
                    }
                }
                MouseButton(button, action, _) => {
                    let mut mouse_queue = scenery
                        .resources
                        .get_mut::<MouseInputStateCollection>()
                        .unwrap();
                    if action == Action::Press {
                        mouse_queue.push(button);
                    } else if action == Action::Release {
                        mouse_queue.pop(button)
                    }
                }
                CursorPos(x, y) => {
                    let mut cursor_pos = scenery
                        .resources
                        .get_mut::<crate::ecs::resources::CursorPos>()
                        .unwrap();
                    cursor_pos.0 = x as f32;
                    cursor_pos.1 = y as f32;
                }
                _ => (),
            }
        }
        !self.win_data.window.should_close()
    }
}
