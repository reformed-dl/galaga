use crate::structs::{COLUMNS, ROWS};
use std::collections::HashMap;
//Components from crossterm crate
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;

pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        GameState {}
    }
    //method that will draw the outline of the game board
    pub fn display_board(&self) {
        //moves the cursor to (0, 0); execute! returns a Result, the unwrap() tells us if we are Ok(()) or an Err()
        execute!(stdout(), MoveTo(0, 0)).unwrap();

        //Step 1-print the top border "+" in top left corner, followed by "-" across the top and "+" in the top right corner
        print!("         +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+         ");//prints "+", finishes top border and ends line

        //Step 2 - print board's content rows "|" followed by blank space and ends with "|"
        for _ in 0..ROWS {
            print!("         |");
            for _ in 0..COLUMNS {
                print!(" ");
            }
            println!("|         ");//prints right side of the border "|" and ends line
        }

        //Step 3 - print the bottom border, repeat step 1
        print!("         +");
        for _ in 0..COLUMNS {
            print!("-");
        }
        println!("+         ");
    }
}