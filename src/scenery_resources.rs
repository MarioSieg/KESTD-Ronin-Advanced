pub use crate::core::platform::prelude::{Action, Key, Modifiers, MouseButton};
use crate::core::platform::prelude::{KEY_COUNT, MOUSE_BUTTON_COUNT};

#[derive(Default, Copy, Clone, Debug)]
pub struct CursorPos(pub f32, pub f32);

#[derive(Clone, Debug)]
pub struct KeyInputStateCollection([bool; KEY_COUNT]);

impl KeyInputStateCollection {
    #[inline]
    pub fn push(&mut self, key: Key) {
        self.0[key as usize] = true
    }

    #[inline]
    pub fn pop(&mut self, key: Key) {
        self.0[key as usize] = false
    }

    #[inline]
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.0[key as usize]
    }
}

impl std::default::Default for KeyInputStateCollection {
    fn default() -> Self {
        Self([false; KEY_COUNT])
    }
}

pub struct MouseInputStateCollection([bool; MOUSE_BUTTON_COUNT]);

impl MouseInputStateCollection {
    #[inline]
    pub fn push(&mut self, key: MouseButton) {
        self.0[key as usize] = true
    }

    #[inline]
    pub fn pop(&mut self, key: MouseButton) {
        self.0[key as usize] = false
    }

    #[inline]
    pub fn is_key_pressed(&self, key: MouseButton) -> bool {
        self.0[key as usize]
    }
}

impl std::default::Default for MouseInputStateCollection {
    fn default() -> Self {
        Self([false; MOUSE_BUTTON_COUNT])
    }
}
