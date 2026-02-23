use core::{
    mem,
    ops::{Index, IndexMut},
};

use crate::{
    game::{
        position::{GenericPosition, Position},
        GameMode,
    },
    LCD,
};

pub struct Sokoban {
    pub level: [[Tile; 4]; 16],
    pub player_position: LevelPosition,
}

impl Default for Sokoban {
    fn default() -> Self {
        let (player_position, level) = decode_level(0);

        Self {
            level,
            player_position,
        }
    }
}

impl Sokoban {
    pub fn draw_full_screen(&self, lcd: &mut LCD) {
        for row in 0..2 {
            lcd.set_cursor(Position::new(0, row));

            for column in 0..16 {
                lcd.write(self.byte_of_tile(Position::new(column, row)));
            }
        }
    }

    pub fn update(
        &mut self,
        lcd: &mut LCD,
        _raw_input: [i8; 2],
        soft_input: [i8; 2],
    ) -> Option<GameMode> {
        None
    }

    pub fn byte_of_tile(&self, position: Position) -> u8 {
        let top_tile = LevelPosition::new(position.column(), position.row() * 2);
        let top = self[top_tile];
        let bottom = self[top_tile.with_row(top_tile.row() + 1)];

        Tile::byte_of_pair(top, bottom)
    }
}

impl Index<LevelPosition> for Sokoban {
    type Output = Tile;

    fn index(&self, index: LevelPosition) -> &Self::Output {
        &self.level[index.column() as usize][index.row() as usize]
    }
}

impl IndexMut<LevelPosition> for Sokoban {
    fn index_mut(&mut self, index: LevelPosition) -> &mut Self::Output {
        &mut self.level[index.column() as usize][index.row() as usize]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Wall,
    Box,
    Destination,
}

impl Tile {
    pub fn byte_of_pair(top: Self, bottom: Self) -> u8 {
        match (top, bottom) {
            (Tile::Empty, Tile::Empty) => b' ',
            (_, Tile::Empty) => 5,
            (Tile::Empty, _) => 6,
            _ => 0xff,
        }
    }
}

type LevelPosition = GenericPosition<2>;

const LEVELS: [(LevelPosition, [u32; 4]); 1] = parse_level(include_bytes!("sokoban.txt"));

pub fn decode_level(index: u8) -> (LevelPosition, [[Tile; 4]; 16]) {
    let &(start, ref level) = &LEVELS[index as usize];

    let mut tiles = [[Tile::Empty; 4]; 16];

    for (row_index, &(mut row)) in level.iter().enumerate() {
        for column_index in (0..16).rev() {
            let tile_bits = row as u8 & 3;
            row >>= 2;

            // SAFETY: tile_bits contains only variations of the two least significant bits, which
            // are all valid Tile discriminants
            let tile = unsafe { mem::transmute::<u8, Tile>(tile_bits) };

            tiles[column_index][row_index] = tile;
        }
    }

    (start, tiles)
}

pub const fn parse_level<const N: usize>(file: &[u8]) -> [(LevelPosition, [u32; 4]); N] {
    let mut level = [(LevelPosition::new(0, 0), [0u32; 4]); N];
    let mut i = 0;

    let mut character = 0;

    while i < file.len() {
        // Skip label
        loop {
            let byte = file[i];
            if byte == b'\n' {
                assert!(file[i - 1] == b':', "Line must end with a `:`");

                i += 1;
                break;
            }
            i += 1;
        }

        let mut start_position = None;

        // Read character
        let mut line_number = 0;
        while line_number < 4 {
            assert!(file[i] == b'|', "Line must start with a `|`");
            i += 1;

            // Read line
            let mut line = 0u32;
            let mut length = 0u32;

            loop {
                let byte = file[i];

                match byte {
                    b' ' => {
                        line <<= 2;
                    }
                    b'#' => {
                        line <<= 2;
                        line |= 1;
                    }
                    b'x' => {
                        line <<= 2;
                        line |= 2;
                    }
                    b'z' => {
                        line <<= 2;
                        line |= 3;
                    }
                    b'p' => {
                        assert!(
                            start_position.is_none(),
                            "Player position can only be defined once per level",
                        );
                        start_position = Some(LevelPosition::new(length as u8, line_number as u8));
                    }
                    b'|' => break,
                    b'\n' => panic!("Line must end with a `|`"),
                    _ => panic!("Line must contain only ` `, `#`, `x`, or `z` between the `|`s"),
                }

                i += 1;
                length += 1;
            }
            assert!(
                length == 16,
                "Line must contain 5 characters between the `|`s"
            );

            assert!(file[i] == b'|', "Line must end with a `|`");
            i += 1;
            assert!(
                i >= file.len() || file[i] == b'\n',
                "Line must end with a `|`"
            );
            i += 1;

            level[character].1[line_number] = line;

            line_number += 1;
        }

        level[character].0 = start_position.expect("Player start position must be defined");

        character += 1;
    }

    level
}
