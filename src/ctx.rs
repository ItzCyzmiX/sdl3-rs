use std::collections::HashSet;

use crate::enums::{Keys, MouseButton};

pub struct Ctx {
    pub dt: f32,
    pub(crate) keys_pressed: HashSet<u32>,
    pub(crate) button_pressed: HashSet<u8>,
    pub(crate) running: bool,
    pub(crate) mouse_pos: (f32, f32),
}

impl Ctx {
    pub(crate) fn new() -> Self {
        Ctx {
            dt: 0.0,
            keys_pressed: HashSet::new(),
            button_pressed: HashSet::new(),
            running: true,
            mouse_pos: (0.0, 0.0),
        }
    }

    pub fn is_key_held(&self, key: Keys) -> bool {
        self.keys_pressed.contains(&(key as u32))
    }

    pub fn is_mouse_held(&self, button: MouseButton) -> bool {
        self.button_pressed.contains(&(button.into()))
    }

    pub fn get_mouse_position(&self) -> (f32, f32) {
        return self.mouse_pos;
    }

    pub fn exit(&mut self) {
        self.running = false
    }
}
