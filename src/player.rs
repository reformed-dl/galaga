use console::{Key, Term};
use crate::structs::{ROWS, COLUMNS, Cords, Timer};
use std::process::exit;

//Player Struct - User's Character, Lives, Location, etc.
pub struct Player {
    display_char: char, //Char represents the character on the screen
    lives: u8, //Number of lives remaining
    current_position: Option<Cords>, //Cords Tuple struct of (usize, usize) provides location and Option of Some or None
    start_position: Cords,//Cords Tuple struct of (usize, usize) gives starting location
    death_timer: Timer,//Timer struct for respawning after death
    key_reader:   
}

impl Player {}