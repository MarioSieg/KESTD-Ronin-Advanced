use super::prelude::*;
use crate::impls::platform::prelude::*;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct KeyInput {
    pub key: Key,
    pub action: Action,
    pub modifier: Modifiers,
}

#[derive(Clone, Debug)]
pub struct MouseInput {
    pub button: MouseButton,
    pub action: Action,
    pub modifier: Modifiers,
}

pub struct PlatformSystem {
    pub win_data: WindowData,
    pub sys_info: SystemInfo,
    pub input_keys_state: SmallVec<[KeyInput; 64]>,
    pub input_mouse_state: SmallVec<[MouseInput; 16]>,
    pub input_string_state: String,
    pub cursor_pos: (f64, f64),
}

impl SubSystem for PlatformSystem {
    type Args = ();

    fn initialize(cfg: &mut CoreConfig, _: &Self::Args) -> Self {
        let sys_info = get_and_print_system_info();
        let win_data = WindowData::create_window(cfg);
        let input_keys_state = SmallVec::new();
        let input_string_state = String::with_capacity(32);
        let input_mouse_state = SmallVec::new();
        let cursor_pos = (0.0, 0.0);

        Self {
            win_data,
            sys_info,
            input_keys_state,
            input_string_state,
            input_mouse_state,
            cursor_pos,
        }
    }

    fn prepare(&mut self) {
        self.win_data.window.focus();
        self.win_data.window.show();
    }

    fn tick(&mut self, _world: &mut World) -> bool {
        self.input_keys_state.clear();
        self.input_mouse_state.clear();
        self.win_data.context.poll_events();
        for (_, event) in flush_messages(&self.win_data.events) {
            use WindowEvent::*;

            match event {
                Key(key, _, action, modifier) => {
                    self.input_keys_state.push(KeyInput {
                        key,
                        action,
                        modifier,
                    });
                }
                MouseButton(button, action, modifier) => self.input_mouse_state.push(MouseInput {
                    button,
                    action,
                    modifier,
                }),
                Char(chr) => self.input_string_state.push(chr),
                CursorPos(x, y) => {
                    self.cursor_pos = (x, y);
                }
                _ => (),
            }
        }
        !self.win_data.window.should_close()
    }
}
