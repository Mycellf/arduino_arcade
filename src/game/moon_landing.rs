use crate::{
    game::{position::Position, GameMode},
    LCD,
};
//use std::num::Float;
use micromath::F32Ext;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
//speed , angle of speed
// level of thrust, angle of thrust
// amount of fuel f99%->f00%
// last corner shows the brawd image of angle and distance to the moon
use ufmt::uwrite;

pub struct MoonLanding {
    ///sim values
    ///
    pub speed: [i16; 2],
    pub acceleration: [i16; 2],

    pub ship_position: [f32; 2],
    pub moon_position: [f32; 2],

    pub fuel: u8,
    pub ship_mass: u16,

    pub distance_to_moon: f32,
    pub gravity: u8,

    //display values
    //pub velosity: u8,
    //pub speedAngle: u8,
   
    //pub thrustPower: u8,

    //pub thrustAngle: u8,
   
    //
    //pub seperation_moon: f32,
}

impl Default for MoonLanding {
    fn default() -> Self {
        Self {
            speed: [300, 0],
            acceleration: [0, 0],
            moon_position: [0.0, 0.0],
            ship_position: [Self::START_POSITION_X, Self::START_POSITION_Y],
            //grav_constent: 1,
            //moon_mass:1,

            gravity: 1, //grav_const and moon_mass replaced with numbers later

            fuel: Self::STARTING_FUEL,  
                     //some number
            ship_mass: Self::BASE_SHIP + Self::STARTING_FUEL as u16,

            distance_to_moon: get_distance(Self::START_POSITION_X, Self::START_POSITION_Y),

           // seperation_moon: get_distance(Self::START_POSITION_X, Self::START_POSITION_X)-Self::MOON_RADIUS as f32,
        }
    }
}

fn get_distance(x: f32, y: f32) -> f32{
    let distance = (x) * (x) + (y)*(y);
    let place_holder=distance.sqrt();

    return place_holder;
}

impl MoonLanding {
    pub const START_POSITION_X: f32= 5000.0;
    pub const START_POSITION_Y: f32= 5000.0;
    pub const GRAVITY_CONSTANT: f32=1.0;
    pub const MOON_MASS:u32=1;//place holder value
    pub const MOON_RADIUS: u32=1;//place holder value
    pub const BASE_SHIP: u16 = 400;// place holder value
    pub const STARTING_FUEL: u8=100;

    pub const DISPLAY_SPEED: Position= Position::new(0, 0);
    //ve
    pub const DISPLAY_THRUST: Position= Position::new(10, 0);
    //ac
    pub const DISPLAY_FUEL: Position= Position::new(0, 1);
    //fu
    pub const DISPLAY_ALTITUDE: Position= Position::new(10, 1);
    //Alt

    fn get_seperation(&self) -> f32 {
        let distance_to_core = get_distance(self.ship_position[0], self.ship_position[1]);

        return distance_to_core - Self::MOON_RADIUS as f32;
    }




    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        //lcd.set_cursor(thrustDisplay);
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
        _: [i8; 2],
    ) -> Option<GameMode> {

        if raw_input[0] != 0 {
            return Some(GameMode::Overworld);
        }

        None
    }
}
