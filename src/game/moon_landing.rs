use crate::{
    game::{position::Position, GameMode},
    rng, utils, LCD,
};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
///speed , angle of speed
/// level of thrust, angle of thrust
/// amount of fuel f99%->f00%
/// last corner shows the brawd image of angle and distance to the moon
use ufmt::uwrite;

pub struct MoonLanding {
    ///sim values
    ///
    pub speed: [i16; 2],
    pub acceleration: [i16; 2],

    pub ship_position: [f32; 2],
    pub moon_position: [f32; 2],

    pub fuel: u8,
    pub empty_ship_mass: u8, //const
    pub ship_mass: u8,

    pub moon_radius: u32,  //const
    pub grav_constent: u8, //const
    pub moon_mass: u8,     //const

    pub distance_to_moon: f32,
    pub gravity: u8,

    ///display values
    ///
    pub speed_display: Position, //const
    //pub velosity: u8,
    //pub speedAngle: u8,
    pub thrust_display: Position, //const
    //pub thrustPower: u8,

    //pub thrustAngle: u8,
    pub fuel_display: Position, //const
    //
    pub seperation_display: Position, //const
    pub seperation_moon: f32,
}

impl Default for MoonLanding {
    fn default() -> Self {
        Self {
            speed: [300, 0],
            acceleration: [0, 0],
            moon_position: [0.0, 0.0],
            ship_position: [500.0, 500.0],
            grav_constent: 1,
            moon_mass:1,

            gravity: grav_constent * moon_mass
                / (ship_position[0] * ship_position[0] + ship_position[1] * ship_position[1]), //grav_const and moon_mass replaced with numbers later

            fuel: 100,           //some number
            empty_ship_mass: 400, //some const number
            ship_mass: empty_shipMass + fuel,

            moon_radius: 3000, //some number
            distance_to_moon: get_distance(ship_position[0], ship_position[1]),

            speed_display: Position::new(1, 0),
            thrust_display: Position::new(0, 0),
            fuel_display: Position::new(0, 8),

            seperation_display: Position::new(1, 8),
            seperation_moon: self.get_seperation(),
        }
    }
}

fn get_distance(x: f32, y: f32) -> f32{
    let distance = ((x) * (x) + (y)(y)).sqrt();
    return distance;
}

impl MoonLanding {
     fn get_seperation(self) -> f32 {
        let distance_to_core = get_distance(ship_position[0], ship_position[1]);

        return distance_to_core - moon_radius;
    }
    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        lcd.set_cursor(thrustDisplay);
        lcd.clear();
        uwrite!(lcd.fmt(), "Hello, spaaace").unwrap_infallible();
        //  lcd.set_cursor(speedDisplay);
        // uwrite!(lcd.fmt(), "m/s {} angle{}", sqrt(speedX*speedX+speedY*speedY), speedAngle ).unwrap_infallible();
        // lcd.set_cursor(thrustDisplay);
        // uwrite!(lcd.fmt(), "thrust{} angle{}", thrustPower, thrustAngle ).unwrap_infallible();
        //  lcd.set_cursor(fuelDisplay);
        // uwrite!(lcd.fmt(), "fuel{}", fuel ).unwrap_infallible();
    }

    pub fn update(
        &mut self,
        lcd: &mut LCD,
        raw_input: [i8; 2],
        soft_input: [i8; 2],
    ) -> Option<GameMode> {
        if raw_input[0] != 0 {
            return return Some(GameMode::Overworld);
        }

        None
    }
}
