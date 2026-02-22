use crate::{game::overworld::Overworld, LCD};

pub mod overworld;
pub mod position;

pub struct Game {
    pub repeat_time: [i8; 2],

    pub overworld: Overworld,
    pub game_mode: GameMode,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            repeat_time: [0; 2],

            overworld: Overworld::default(),
            game_mode: GameMode::Overworld,
        }
    }
}

pub enum GameMode {
    Overworld,
}

impl Game {
    pub const REPEAT_DELAY_FRAMES: i8 = 15;

    pub fn start(&mut self, lcd: &mut LCD) {
        self.overworld.start(lcd);
    }

    pub fn update(&mut self, lcd: &mut LCD, raw_input: [i8; 2]) {
        let soft_input = self.update_soft_input(raw_input);

        match self.game_mode {
            GameMode::Overworld => {
                self.overworld.update(lcd, raw_input, soft_input);
            }
        }
    }

    pub fn update_soft_input(&mut self, input: [i8; 2]) -> [i8; 2] {
        [0, 1].map(|i| {
            if input[i] > 0 {
                if self.repeat_time[i] > 0 {
                    self.repeat_time[i] -= 1;
                } else {
                    self.repeat_time[i] = Self::REPEAT_DELAY_FRAMES;
                    return 1;
                }
            } else if input[i] < 0 {
                if self.repeat_time[i] < 0 {
                    self.repeat_time[i] += 1;
                } else {
                    self.repeat_time[i] = -Self::REPEAT_DELAY_FRAMES;
                    return -1;
                }
            } else {
                self.repeat_time[i] = 0;
            }

            0
        })
    }
}
