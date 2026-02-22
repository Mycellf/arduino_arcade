#![no_std]
#![no_main]

pub mod characters;
pub mod lcd;
pub mod overworld;

use arduino_hal::hal::port::{PB2, PB3, PB4, PD2, PD3, PD4, PD5};
use panic_halt as _;

use crate::{
    characters::CHARACTERS,
    lcd::{
        options::{FontSize, NumLines},
        LCDInfo,
    },
};

/// That's too many to type out all the time
pub type LCD = lcd::LCD<PB4, PB2, PB3, PD5, PD4, PD3, PD2>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut lcd = LCD {
        register_select: pins.d12.into_output(),
        read_write: pins.d10.into_output(),
        enable: pins.d11.into_output(),

        data_4: pins.d5.into_output(),
        data_5: pins.d4.into_output(),
        data_6: pins.d3.into_output(),
        data_7: pins.d2.into_output(),

        info: LCDInfo::new(16, NumLines::Two, FontSize::Dots5x8),
    };

    lcd.begin();

    for (i, character) in CHARACTERS.iter().enumerate() {
        lcd.create_character(i as u8, character);
    }

    overworld::print_screen(&mut lcd, 0);

    loop {
        arduino_hal::delay_ms(1000);
    }
}
