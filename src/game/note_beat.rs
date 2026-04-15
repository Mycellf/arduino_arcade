use crate::{
    game::{position::Position, GameMode},
    rng::rng,
    LCD,
};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use ufmt::uwrite;

// what I need to add hit detdection
// for self and objects
// score

pub struct NoteBeat {
    pub objects: [Object; 16],
    pub difficulty: u8,
    pub time: u8,
    pub time_gap: u8,
    pub min_speed: u8,
    pub strike_latency: u8,
    pub player_position: Position,
    pub player_hit: bool,
    pub redraw: bool,
    pub lives: u8,

    pub combo: u16,
    pub score: u32,
}

impl Default for NoteBeat {
    fn default() -> Self {
        Self {
            objects: [Object::None; 16],
            difficulty: 10,
            time: 0,
            time_gap: 60,
            min_speed: 20,
            strike_latency: 0,
            player_position: Self::START_POSITION,

            player_hit: false,
            redraw: false,
            lives: 3,
            combo: 1,
            score: 0,
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Object {
    None = b' ',
    Default = b'*',
    DownEnemy = b'V',
    UpEnemy = b'A',
    Strike = b'o',
}

impl Object {
    pub fn vulnerable_to_input(self, input_y: i8) -> bool {
        match self {
            Object::Default => true,
            Object::UpEnemy => input_y < 0,
            Object::DownEnemy => input_y > 0,
            _ => false,
        }
    }

    pub fn is_enemy(self) -> bool {
        matches!(self, Object::Default | Object::DownEnemy | Object::UpEnemy)
    }
}

impl NoteBeat {
    pub const LEFT_POSITION: Position = Position::new(0, 0);
    pub const START_POSITION: Position = Position::new(7, 0);
    pub const STRIKE_LATENCY: u8 = 5;

    pub const PLAYER_CHARACTER: u8 = 0;

    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        lcd.clear();

        lcd.set_cursor(Position::new(0, 0));
        for enemy in self.objects {
            lcd.write(enemy as u8);
        }

        lcd.set_cursor(self.player_position);
        lcd.write(Self::PLAYER_CHARACTER);

        lcd.set_cursor(Self::LEFT_POSITION.nudge_row_saturating(1));
        uwrite!(lcd.fmt(), "score").unwrap_infallible();
        lcd.set_cursor(
            Self::LEFT_POSITION
                .nudge_row_saturating(1)
                .nudge_column_saturating(8),
        );
        uwrite!(lcd.fmt(), " combo").unwrap_infallible();
    }

    pub fn add_to_queue(&mut self) {
        let random = rng() % self.difficulty as u32;

        self.move_objects();

        if random < 4 {
            if random % 2 == 0 {
                self.objects[0] = self.add_object();
                self.objects[15] = Object::None;
            } else {
                self.objects[0] = Object::None;
                self.objects[15] = self.add_object();
            }
        }
    }

    fn add_object(&mut self) -> Object {
        let random = rng() % self.difficulty as u32;
        match random {
            0 | 1 => return Object::UpEnemy,
            2 | 3 => return Object::DownEnemy,
            _ => return Object::Default,
        }
    }

    fn move_objects(&mut self) {
        for i in 0..7 {
            self.objects[7 - i] = self.objects[6 - i];
        }
        for i in 8..15 {
            self.objects[i] = self.objects[i + 1];
        }

        if self.objects[7].is_enemy() || self.objects[8].is_enemy() {
            self.player_hit = true;
        }
    }

    fn hit_object(&mut self, raw_input: [i8; 2]) {
        let input_x = raw_input[0];
        let input_y = raw_input[1];

        let left = input_x < 0 && self.objects[6].vulnerable_to_input(input_y);
        let right = input_x > 0 && self.objects[9].vulnerable_to_input(input_y);

        let hit_someting: bool = left ^ right;

        if hit_someting {
            if input_x < 0 {
                self.objects[6] = Object::Strike;
            } else {
                self.objects[9] = Object::Strike;
            }
            self.score = self.score + self.combo as u32;
            self.combo = self.combo + 1;
            self.redraw = true;
            self.strike_latency = Self::STRIKE_LATENCY;
        } else {
            self.combo = 1;
            self.strike_latency = Self::STRIKE_LATENCY * 2;
        }
    }

    fn draw_objects(&mut self, lcd: &mut LCD) {
        lcd.set_cursor(Self::LEFT_POSITION);
        for object in self.objects {
            lcd.write(object as u8);
        }
    }

    pub fn update(
        &mut self,
        lcd: &mut LCD,
        raw_input: [i8; 2],
        _soft_input: [i8; 2],
    ) -> Option<GameMode> {
        lcd.set_cursor(Self::LEFT_POSITION);

        if self.time % self.time_gap == 0 {
            if (self.time_gap as u32 - self.score / 10) > self.min_speed as u32 {
                self.time_gap = self.time_gap - (self.score as u8) / 10;
            }
            self.add_to_queue();
            self.redraw = true;
            self.time = 1;
        } else {
            self.time = self.time + 1;
        }

        let mut redraw_player = false;

        if raw_input[0] != 0 {
            if self.strike_latency == 0 {
                self.hit_object(raw_input);
            } else {
                self.strike_latency = self.strike_latency - 1;
            }

            let new_positon = if raw_input[0] > 0 {
                // Right
                Self::START_POSITION.nudge_column_overflowing(1).0
            } else {
                // Left
                Self::START_POSITION
            };

            if self.player_position != new_positon {
                redraw_player = true;
            }

            self.player_position = new_positon;
        }

        if self.redraw {
            self.redraw = false;

            self.draw_objects(lcd);

            lcd.set_cursor(Position::new(5, 1));
            uwrite!(lcd.fmt(), "{}", self.score).unwrap_infallible();

            lcd.set_cursor(Position::new(1, 13));
            uwrite!(lcd.fmt(), "{}", self.combo).unwrap_infallible();
            // TODO: Clear the space after if the combo decreases

            redraw_player = true;
        }

        if redraw_player {
            lcd.set_cursor(Self::START_POSITION);

            if self.player_position == Self::START_POSITION {
                lcd.write(Self::PLAYER_CHARACTER);
                lcd.write(b' ');
            } else {
                lcd.write(b' ');
                lcd.write(Self::PLAYER_CHARACTER);
            }
        }

        if self.player_hit {
            self.player_hit = false;

            if self.lives > 0 {
                self.lives = self.lives - 1;
                self.objects = [Object::None; 16];
                lcd.set_cursor(Self::LEFT_POSITION);
                for _ in 0..16 {
                    lcd.write(b'x');
                }
            } else {
                return Some(GameMode::Overworld);
            }
        }

        None
    }
}
