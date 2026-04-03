//just the beats game?
//something randomly adding to a 8 size arrray 
//that is then split to either left or right bolth size 8
//every time increment moves notes up
//hit the right note and an increment hapens reseting time 
//dificulty is number in the corner slowly going down, the time left to make an action 

//use core::ops::{Index, IndexMut, Range};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use ufmt::uwrite;

use crate::{
    game::{position::Position, GameMode},
    rng, utils, LCD,
};


pub struct NoteBeat{
    pub position_left: Position,
    pub position_self: Position,
    pub generated_array: [Directions; 8],
    //pub size_of_array: u8,



    // pub spawn_timer: u8,
    // pub move_timer: u8,
    // pub score: u8,
    // generated_array:[directions; 8], 
//array of positons or somthing for 
}

impl Default for NoteBeat{
    fn default() -> Self{
        //uwrite!(serial, "Hello world noteBeat");
        Self{
            position_left: Position::new(0, 0),
            position_self: Position::new(7, 1),
            generated_array: [none; 8],
            
            // spawn_timer: 0,
            // move_timer: 0,
            // score: 0,
            

        }

    }

}
pub enum Directions{
    left, right, none,

    
}

impl NoteBeat {
    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
    
        lcd.set_cursor(self.position_left);
        lcd.clear();
        uwrite!(lcd.fmt(), "Hello, world!").unwrap_infallible();
    }

    fn parse_queue(direction_list: [Directions; 8])-> &str {
        let mut name="";
        
        for i in 0..size_of(){
            if(direction_list[i]==Directions::right){
                name = name + "R";
            }else{
                name = name + " ";
            }
            if(direction_list[i]==Directions::left){
                name = "L"+ name;
            }else{
                name = " "+ name;
            }
        }
        return name;

    }

    pub fn update(&mut self, lcd: &mut LCD, raw_input: [i8; 2], soft_input: [i8; 2]) -> Option<GameMode> {
        
        lcd.set_cursor(self.position_left);
        lcd.clear();
        let incoming = parse_queue(generated_array);
        uwrite!(lcd.fmt(), "{}", incoming).unwrap_infallible();
        lcd.set_cursor(self.position_self);
        uwrite!(lcd.fmt(), "pl").unwrap_infallible();

        if raw_input[0]> 0 {
        uwrite!(lcd.fmt(), "0+").unwrap_infallible();

        }else if raw_input[1]> 0  {
        uwrite!(lcd.fmt(), "1+").unwrap_infallible();
        }


        None
    }
}
