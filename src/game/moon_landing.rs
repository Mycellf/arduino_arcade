///speed , angle of speed 
/// level of thrust, angle of thrust 
/// amount of fuel f99%->f00% 
/// last corner shows the brawd image of angle and distance to the moon 
use ufmt::uwrite;

use crate::{
    game::{position::Position, GameMode},
    rng, utils, LCD,
};

pub struct MoonLanding{
    ///sim values
    /// 
    pub speed: [i16; 2],
   // pub speedX: u8,
    //pub speedY: u8,

    pub acceleration: [i16; 2],
   // pub thrustX:u8,
    //pub thrustY:u8,

    pub ship_position:  [f32; 2],
    pub moon_position:  [f32; 2],
    
    pub fuel: u8,
    pub empty_shipMass:u8,
    pub shipMass: u8,

    //const radius moon
    pub seperation_moon: f32,
    pub gravity: u8,
    pub grav_constent: u8,
    pub moon_mass: u8,
   


    ///display values
    /// 
    pub speed_display: Position,
    //pub velosity: u8,
    //pub speedAngle: u8,
    pub thrust_display: Position,
    //pub thrustPower: u8,
    //pub thrustAngle: u8,
    pub fuel_display: Position,
    //
    pub Seperation_display: Position,
    //

}


impl Default for MoonLanding{
    fn default() -> Self{

        Self{
            speed: [300, 0],
            acceleration: [0,0],
            moon_position: [0,0],
            ship_position: [5000, 5000],
            gravity: grav_constent*moon_mass(ship_position[0]+ship_position[1]), //grav_const and moon_mass replaced with numbers later

            fuel: 100, //some number
            empty_shipMass: 400,//some const number 
            shipMass: empty_shipMass+fuel,

            seperation_moon: 



            

            speed_display: Position::new(1, 0),
            thrust_display: Position::new(0, 0),
            fuel_display: Position::new(0, 8),
            Seperation_display: Position::new(1, 8),
            
        }

    }

    pub fn get_seperation(){
        
    }
}

impl MoonLanding {
    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
        lcd.set_cursor(thrustDisplay);
        lcd.clear();
        uwrite!(lcd.fmt(), "Hello, spaaace").unwrap_infallible();
        ///  lcd.set_cursor(speedDisplay);
        /// uwrite!(lcd.fmt(), "m/s {} angle{}", sqrt(speedX*speedX+speedY*speedY), speedAngle ).unwrap_infallible();
        /// lcd.set_cursor(thrustDisplay);
        /// uwrite!(lcd.fmt(), "thrust{} angle{}", thrustPower, thrustAngle ).unwrap_infallible();
        ///  lcd.set_cursor(fuelDisplay);
        /// uwrite!(lcd.fmt(), "fuel{}", fuel ).unwrap_infallible();


    }

    pub fn update(&mut self, lcd: &mut LCD, raw_input: [i8; 2], soft_input: [i8; 2]) -> Option<GameMode> {

        if raw_input[0]!=0{
            return return Some(GameMode::Overworld);
        }
        

        None
    }

    

}
