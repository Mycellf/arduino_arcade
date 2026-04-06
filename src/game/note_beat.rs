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
    LCD, game::{GameMode, position::Position}, rng::{self, rng}, utils
};


pub struct NoteBeat{
    pub position_left: Position,
    pub position_self: Position,
    pub generated_array: [Directions; 8],
    pub difficulty: u32,
    pub time: u8,
    pub time_gap:u8,
    pub got_hit:bool,
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
            generated_array: [Directions::empty; 8],
            difficulty: 8,
            time:0,
            time_gap: 30,
            got_hit: false,
            
            // spawn_timer: 0,
            // move_timer: 0,
            // score: 0,
            

        }

    }

}
#[derive(Copy, Clone)]
pub enum Directions{
    left, right, empty,

    
}

impl NoteBeat {
    pub fn draw_full_screen(&mut self, lcd: &mut LCD) {
    
        lcd.set_cursor(self.position_left);
        lcd.clear();
        uwrite!(lcd.fmt(), "Hello, world!").unwrap_infallible();
    }

    pub fn parse_queue( &mut self) -> &str {
        let mut name="";
        for i in 0..8{
            name = match self.generated_array[i] {
                Directions::right => " {name}R",
                Directions::left =>  "L{name} ",
                Directions::empty =>  " {name} ",
            };
        }
                
        return name;

    }

    pub fn add_to_queue(&mut self){
        let num = rng()%self.difficulty;
        for i in 0..7{
            self.generated_array[i] = self.generated_array[i+1];
        }
         self.generated_array[7]= match num{
            0=> Directions::left,
            1=>Directions::right,
            2=> Directions::left,
            3=>Directions::right,
            _=> Directions::empty,
        }
    }

    pub fn update(&mut self, lcd: &mut LCD, raw_input: [i8; 2], soft_input: [i8; 2]) -> Option<GameMode> {
        
        lcd.set_cursor(self.position_left);

        if self.time%self.time_gap == 0{
            self.add_to_queue();
            self.time=0;
            lcd.clear();

        }else{
            self.time=self.time+1;
        }

        let display_text=self.parse_queue();
       // self.parse_queue();
        uwrite!(lcd.fmt(), "{}", display_text).unwrap_infallible();
        lcd.set_cursor(self.position_self);
        uwrite!(lcd.fmt(), "pl").unwrap_infallible();

        if raw_input[0]> 0 {//right
            lcd.set_cursor(Position::new(8, 0));
        uwrite!(lcd.fmt(), ">").unwrap_infallible();
        //if item hit set frount of queue to empty and time to 28 

        }else if raw_input[0]< 0  {//left
            lcd.set_cursor(Position::new(7, 0));
        uwrite!(lcd.fmt(), "<").unwrap_infallible();
        //if item hit set frount of queue to empty and time to 28
        }else if raw_input[1]< 0{
            //exit

        }



        None
    }
}
