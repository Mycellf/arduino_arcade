//use core::random;

use crate::{
    game::{position::Position, GameMode},
    lcd::characters,
    rng::rng,
    LCD,
};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use ufmt::uwrite;

// dificulty curve //speed yes, other factors no
// miss efects //done
// combo fix //done
//
//

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

    //pub combo: u16,
    pub score: u32,
}

impl Default for NoteBeat {
    fn default() -> Self {
        Self {
            objects: [Object::None; 16],
            difficulty: 1,
            time: 0,
            time_gap: 60,
            min_speed: 20,
            strike_latency: 0,
            player_position: Self::START_POSITION,

            player_hit: false,
            redraw: false,
            lives: 3,
            //combo: 1,
            score: 0,
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Object {
    None = b' ',
    NoneMiss = b'o',
    Default = 3,
    DownEnemy = 2,
    DownEnemyMiss = 5,
    UpEnemy = 1,
    UpEnemyMiss = 4,
    Strike = b'X',
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
        matches!(
            self,
            Object::Default
                | Object::DownEnemy
                | Object::UpEnemy
                | Object::DownEnemyMiss
                | Object::UpEnemyMiss
        )
    }

    pub fn get_miss(self) -> Object {
        match self {
            Object::None => Object::NoneMiss,
            Object::DownEnemy => Object::DownEnemyMiss,
            Object::UpEnemy => Object::UpEnemyMiss,
            _ => self,
        }
    }
}

impl NoteBeat {
    pub const LEFT_POSITION: Position = Position::new(0, 0);
    pub const START_POSITION: Position = Position::new(7, 0);
    pub const STRIKE_LATENCY: u8 = 5;
    pub const START_SPEED: u8 = 60;

    pub const PLAYER_CHARACTER: u8 = 0;

    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        // TODO:
        characters::load_character_set(lcd, 3);
        lcd.clear();

        lcd.set_cursor(Position::new(0, 0));
        for enemy in self.objects {
            lcd.write(enemy as u8);
        }

        lcd.set_cursor(self.player_position);
        lcd.write(Self::PLAYER_CHARACTER);

        lcd.set_cursor(Position::new(0, 1));
        uwrite!(lcd.fmt(), "score").unwrap_infallible();
        lcd.set_cursor(Position::new(8, 1));
        uwrite!(lcd.fmt(), "lives").unwrap_infallible();
        lcd.set_cursor(Position::new(13, 1));
        for _ in 0..self.lives {
            {
                lcd.write(6);
            }
        }
    }

    pub fn add_to_queue(&mut self) {
        let random = rng() % (5 - self.difficulty) as u32;

        self.move_objects();

        match random {
            0 => {
                self.objects[0] = self.add_object();
                self.objects[15] = Object::None;
            }
            1 => {
                self.objects[0] = Object::None;
                self.objects[15] = self.add_object();
            }
            _ => {
                self.objects[0] = Object::None;
                self.objects[15] = Object::None;
            }
        }
    }

    fn add_object(&mut self) -> Object {
        let random = rng() % 4;
        match random {
            0 => return Object::UpEnemy,
            1 => return Object::DownEnemy,
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

        let hit_location = match input_x {
            -1 => 6,
            1 => 9,
            _ => unreachable!(),
        };

        let hit_someting = self.objects[hit_location].vulnerable_to_input(input_y);
        if !hit_someting {
            self.objects[hit_location] = self.objects[hit_location].get_miss();
        } else {
            self.objects[hit_location] = Object::Strike;
            self.score = self.score + 1;
        }

        self.redraw = true;
        self.strike_latency = Self::STRIKE_LATENCY;
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
            if (Self::START_SPEED as u32 - (self.score + 2).ilog2() * 4) > self.min_speed as u32 {
                self.time_gap = 60 - ((self.score + 2).ilog2() * 4) as u8;
            } else {
                self.time_gap = self.min_speed;
                self.min_speed = self.min_speed - 1;
            }

            self.add_to_queue();
            self.redraw = true;
            self.time = 1;
            self.score = self.score + 1;
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

            redraw_player = true;
        }

        if redraw_player {
            lcd.set_cursor(Self::START_POSITION);

            if self.player_position == Self::START_POSITION {
                lcd.write(Self::PLAYER_CHARACTER);
                lcd.write(b'_');
            } else {
                lcd.write(b'_');
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
                lcd.set_cursor(Position::new(13, 1));
                for i in 0..3 {
                    if i < self.lives {
                        lcd.write(6);
                    } else {
                        lcd.write(b'X');
                    }
                }
                self.min_speed = 60;
                self.time = 1;
            } else {
                characters::load_character_set(lcd, 0);

                return Some(GameMode::Overworld);
            }
        }

        None
    }
}
