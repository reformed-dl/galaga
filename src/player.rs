use console::{Key, Term};
use crate::structs::{ROWS, COLUMNS, Cords, Timer};
use std::process::exit;

//Player Struct - User's Character, Lives, Location, etc.
pub struct Player {
    pub display_char: char, //Char represents the character on the screen
    pub lives: u8, //Number of lives remaining
    pub current_position: Option<Cords>, //Cords Tuple struct of (usize, usize) provides location and Option of Some or None
    pub start_position: Cords,//Cords Tuple struct of (usize, usize) gives starting location
    pub death_timer: Timer,//Timer struct for respawning after death
    pub key_reader: KeyReader,   
}

impl Player {
    pub fn new() -> Self {
        let start_position = Cords(ROWS - 2, COLUMNS/2);
        Player {
            display_char: '^',
            lives: 4,
            current_position: Some(start_position),
            start_position,
            death_timer: Timer::new(200),
            key_reader: KeyReader,


        }
    }
}