use core::{
    mem,
    ops::{Index, IndexMut},
};

use crate::{
    game::{position::Position, GameMode},
    utils::List,
    LCD,
};

pub struct Minesweeper {
    pub cursor_position: Position,
    pub tiles: [[Tile; 16]; 2],

    pub clicked_any: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub covered: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            kind: TileKind::Num0,
            covered: true,
        }
    }
}

impl Tile {
    pub fn as_byte(self) -> u8 {
        if self.covered {
            0x07
        } else {
            self.kind as u8
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum TileKind {
    Mine = b'*',
    Num0 = b' ',
    Num1 = b'1',
    Num2 = b'2',
    Num3 = b'3',
    Num4 = b'4',
    Num5 = b'5',
    Num6 = b'6',
    Num7 = b'7',
    Num8 = b'8',
    Num9 = b'9',
}

impl TileKind {
    pub fn increment_count(&mut self) {
        match self {
            Self::Mine | Self::Num9 => (),
            Self::Num0 => *self = Self::Num1,
            _ => {
                // SAFETY: We've checked that the next index is safe
                *self = unsafe { mem::transmute(*self as u8 + 1) };
            }
        }
    }
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            cursor_position: Position::new(0, 0),
            tiles: [[Tile::default(); 16]; 2],

            clicked_any: false,
        }
    }
}

impl Index<Position> for Minesweeper {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.tiles[index.row() as usize][index.column() as usize]
    }
}

impl IndexMut<Position> for Minesweeper {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.tiles[index.row() as usize][index.column() as usize]
    }
}

impl Minesweeper {
    pub const CURSOR_CHARACTER: u8 = b'_';
    pub const NUM_MINES: u8 = 8;

    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        lcd.clear();
        for (i, row) in self.tiles.iter().enumerate() {
            lcd.set_cursor(Position::new(0, i as u8));
            for &tile in row {
                lcd.write(tile.as_byte());
            }
        }

        lcd.set_cursor(self.cursor_position);
        lcd.write(Self::CURSOR_CHARACTER);
    }

    pub fn update(
        &mut self,
        lcd: &mut LCD,
        _raw_input: [i8; 2],
        soft_input: [i8; 2],
    ) -> Option<GameMode> {
        if soft_input != [0; 2] {
            let (new_cursor_position, click) = self
                .cursor_position
                .nudge_column_saturating(soft_input[0])
                .nudge_row_overflowing(soft_input[1]);

            if click {
                let cursor_position = self.cursor_position;
                self[cursor_position].covered = false;

                if !self.clicked_any {
                    self.clicked_any = true;
                    self.first_click();
                }

                lcd.set_cursor(self.cursor_position);
                lcd.write(self[self.cursor_position].as_byte());
            } else if new_cursor_position != self.cursor_position {
                lcd.set_cursor(self.cursor_position);
                lcd.write(self[self.cursor_position].as_byte());

                self.cursor_position = new_cursor_position;

                lcd.set_cursor(self.cursor_position);
                lcd.write(Self::CURSOR_CHARACTER);
            }
        }

        None
    }

    pub fn player_won(&self) -> bool {
        true
    }

    pub fn first_click(&mut self) {
        let mut positions = List::<Position, 31>::new();
        for row in 0..2 {
            for column in 0..16 {
                let position = Position::new(column, row);
                if position == self.cursor_position {
                    continue;
                }
                positions.insert(position).unwrap();
            }
        }

        for _ in 0..Self::NUM_MINES {
            self.insert_mine(positions.remove_random().unwrap());
        }
    }

    pub fn insert_mine(&mut self, position: Position) {
        if self[position].kind == TileKind::Mine {
            return;
        }

        self[position].kind = TileKind::Mine;

        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }

                let (position, side_edge) = position.nudge_column_overflowing(x);
                let (position, top_edge) = position.nudge_row_overflowing(y);
                if side_edge || top_edge {
                    continue;
                }

                self[position].kind.increment_count();
            }
        }
    }
}
