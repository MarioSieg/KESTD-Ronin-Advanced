pub use super::sys_info::{get_and_print_system_info, SystemInfo};
pub use super::WindowData;
pub use glfw::flush_messages;
pub use glfw::Window as WindowHandle;
pub use glfw::{Action, Key, Modifiers, MouseButton, WindowEvent};
pub use std::sync::mpsc::Receiver;

pub const KEY_COUNT: usize = Key::Menu as usize + 1;
pub const MOUSE_BUTTON_COUNT: usize = MouseButton::Button8 as usize + 1;
