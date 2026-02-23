use core::{mem, num::NonZeroU8};

use crate::{
    game::{position::Position, GameMode},
    utils::List,
    LCD,
};

pub struct BlackJack {
    pub table: [Option<Card>; Self::TABLE_WIDTH as usize],

    pub num_player_cards: u8,
    pub player_points: u8,

    pub num_dealer_cards: u8,
    pub dealer_points: u8,

    pub player_position: u8,

    pub deck: Deck<52>,
}

impl Default for BlackJack {
    fn default() -> Self {
        Self {
            table: [None; Self::TABLE_WIDTH as usize],

            num_player_cards: 0,
            player_points: 0,

            num_dealer_cards: 0,
            dealer_points: 0,

            player_position: 0,

            deck: Deck::full(),
        }
    }
}

const _: () = assert!(BlackJack::MAXIMUM_CARDS <= 52);

impl BlackJack {
    pub const PLAYER_CHARACTER: u8 = 0;

    pub const TABLE_WIDTH: u8 = 12;
    pub const MAXIMUM_CARDS: u8 = Self::TABLE_WIDTH - 1;

    pub const TABLE_START_COLUMN: u8 = 4;

    pub fn draw_full_screen(&self, lcd: &mut LCD) {
        lcd.clear();

        lcd.set_cursor(Position::new(self.player_position, 0));
        lcd.write(Self::PLAYER_CHARACTER);
        lcd.set_cursor(Position::new(0, 1));
        lcd.print_bytes(b"HS");
    }

    pub fn update(
        &mut self,
        lcd: &mut LCD,
        _raw_input: [i8; 2],
        soft_input: [i8; 2],
    ) -> Option<GameMode> {
        match soft_input[0] {
            1 => self.set_player_position(lcd, 1),
            -1 => self.set_player_position(lcd, 0),
            _ => (),
        }

        None
    }

    pub fn set_player_position(&mut self, lcd: &mut LCD, position: u8) {
        if self.player_position == position {
            return;
        }

        lcd.set_cursor(Position::new(self.player_position, 0));
        lcd.write(b' ');
        if self.player_position + 1 != position {
            lcd.set_cursor(Position::new(position, 0));
        }
        lcd.write(Self::PLAYER_CHARACTER);

        self.player_position = position;
    }

    pub fn table_full(&mut self) -> bool {
        self.num_player_cards + self.num_dealer_cards >= Self::MAXIMUM_CARDS
    }

    pub fn add_player_card(&mut self, lcd: &mut LCD) -> Option<()> {
        if self.table_full() {
            return None;
        }

        let card = self.deck.remove_random().unwrap();

        let index = self.num_player_cards;
        card.draw_at(lcd, index + Self::TABLE_START_COLUMN);
        self.table[index as usize] = Some(card);

        self.num_player_cards += 1;

        Some(())
    }

    pub fn add_dealer_card(&mut self, lcd: &mut LCD) -> Option<()> {
        if self.table_full() {
            return None;
        }

        let card = self.deck.remove_random().unwrap();

        self.num_dealer_cards += 1;

        let index = Self::TABLE_WIDTH - self.num_dealer_cards;
        card.draw_at(lcd, index + Self::TABLE_START_COLUMN);
        self.table[index as usize] = Some(card);

        Some(())
    }

    pub fn player_won(&self) -> bool {
        self.player_points > self.dealer_points
    }
}

pub type Deck<const N: usize> = List<Card, N>;

impl Deck<52> {
    pub fn full() -> Self {
        let mut deck = Self::new();

        for suit_bits in 1..4 + 1 {
            for number_bits in (0x00..0xc0 + 0x10).step_by(0x10) {
                deck.insert(unsafe { Card::from_bits_unchecked(number_bits | suit_bits) })
                    .unwrap();
            }
        }

        assert_eq!(deck.len(), 52);

        deck
    }
}

/// INVARIANT: Must contain the a `Number`'s bits bitwise or'ed with a `Suit`'s bits
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Card(NonZeroU8);

impl Card {
    pub fn new(number: Number, suit: Suit) -> Self {
        // SAFETY: `suit as u8` is always nonzero
        Self(unsafe { NonZeroU8::new_unchecked(number as u8 | suit as u8) })
    }

    pub unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { Self(NonZeroU8::new_unchecked(bits)) }
    }

    pub fn number(self) -> Number {
        // SAFETY: The 4 most significant bits must have been set from a Number
        unsafe { mem::transmute::<u8, Number>(self.0.get() & 0xf0) }
    }

    pub fn suit(self) -> Suit {
        // SAFETY: The 4 least significant bits must have been set from a Suit
        unsafe { mem::transmute::<u8, Suit>(self.0.get() & 0x0f) }
    }

    pub fn draw_at(self, lcd: &mut LCD, column: u8) {
        lcd.set_cursor(Position::new(column, 0));
        lcd.write(self.suit().character());
        lcd.set_cursor(Position::new(column, 1));
        lcd.write(self.number().character());
    }
}

/// NOTE: Only occupies the 4 most significant bits
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Number {
    Ace = 0x00,
    Number2 = 0x10,
    Number3 = 0x20,
    Number4 = 0x30,
    Number5 = 0x40,
    Number6 = 0x50,
    Number7 = 0x60,
    Number8 = 0x70,
    Number9 = 0x80,
    Number10 = 0x90,
    Jack = 0xa0,
    Queen = 0xb0,
    King = 0xc0,
}

/// NOTE: Only occupies the 4 least significant bits
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Suit {
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
    Clubs = 4,
}

impl Number {
    pub fn character(self) -> u8 {
        match self {
            Number::Ace => b'A',
            Number::Number2 => b'2',
            Number::Number3 => b'3',
            Number::Number4 => b'4',
            Number::Number5 => b'5',
            Number::Number6 => b'6',
            Number::Number7 => b'7',
            Number::Number8 => b'8',
            Number::Number9 => b'9',
            Number::Number10 => b't',
            Number::Jack => b'J',
            Number::Queen => b'Q',
            Number::King => b'K',
        }
    }
}

impl Suit {
    pub fn character(self) -> u8 {
        self as u8
    }
}
