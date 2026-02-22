use core::fmt::Write;

use crate::LCD;

pub mod overworld;

pub struct Game {}

impl Default for Game {
    fn default() -> Self {
        Self {}
    }
}

impl Game {
    pub fn update(&mut self, lcd: &mut LCD, input: [i8; 2]) {
        lcd.set_cursor(0, 0);
        write!(lcd.fmt(), "{input:?}  ").unwrap();
    }
}
