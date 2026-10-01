use std::collections::HashSet;

use crate::enums::Keys;

pub struct Ctx {
    pub dt: f32,
    pub(crate) keys_pressed: HashSet<u32>,
    pub(crate) running: bool,
}

impl Ctx {
    pub(crate) fn new() -> Self {
        Ctx {
            dt: 0.0,
            keys_pressed: HashSet::new(),
            running: true,
        }
    }

    pub fn is_key_held(&self, key: Keys) -> bool {
        self.keys_pressed.contains(&(key as u32))
    }

    pub fn exit(&mut self) {
        self.running = false
    }
}
