use crate::{
    game::{position::Position, GameMode},
    LCD,
};

pub struct BlockCatch {
    pub player_position: Position,
}

impl Default for BlockCatch {
    fn default() -> Self {
        Self {
            player_position: Position::new(0, 0),
        }
    }
}

impl BlockCatch {
    pub const PLAYER_CHARACTER: u8 = 0;

    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        lcd.clear();

        lcd.set_cursor(self.player_position);
        lcd.write(Self::PLAYER_CHARACTER);
    }

    pub fn update(&mut self, lcd: &mut LCD, raw_input: [i8; 2]) -> Option<GameMode> {
        self.move_player_by(lcd, raw_input);

        None
    }

    pub fn move_player_by(&mut self, lcd: &mut LCD, input: [i8; 2]) -> bool {
        let mut new_position = self.player_position;

        match input[0] {
            1 => new_position = new_position.with_column(1),
            -1 => new_position = new_position.with_column(0),
            _ => (),
        }

        match input[1] {
            1 => new_position = new_position.with_row(1),
            -1 => new_position = new_position.with_row(0),
            _ => (),
        }

        if new_position == self.player_position {
            return false;
        }

        lcd.set_cursor(self.player_position);
        lcd.write(b' '); // TODO: Keep whatever marker was there

        lcd.set_cursor(new_position);
        lcd.write(Self::PLAYER_CHARACTER);

        self.player_position = new_position;

        true
    }
}
