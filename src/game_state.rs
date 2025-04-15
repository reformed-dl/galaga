use crate::structs::{COLUMNS, ROWS};
use std::collections::HashMap;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;

pub struct GameState {

}

impl GameState {
    pub fn new() -> Self {
        GameState {}
    }
    //method that will draw the outline of the game board
    pub fn display_board(&self) {
        execute!(stdout(), MoveTo(0, 0)).unwrap();//moves the cursor to (0, 0); execute! returns a Result, the unwrap() tells us if we are Ok(()) or an Err()
        print!("+");//prints "+" at (0, 0)
        for x in 1..COLUMNS {
            print!("-");//columns value is usize = 20, 0 - 19, this should print "-" from 1 - 18
        }
        execute!(stdout(), MoveTo((COLUMNS - 1), 0)).unwrap();//moves cursor to final column
        print!("+");

        for y in 1..(ROWS - 1) {//row 0 is written with code above, Print "|" in ROWS 1 - 8
            execute!(stdout(), MoveTo(0, y)).unwrap();//move cursor to Row 1 (0, 1)
            print!("|");
            execute!(stdout(), MoveTo((COLUMNS - 1), y)).unwrap();//move cursor to last column and print "|"
            print!("|");
        }

        execute!(stdout(), MoveTo(0, (ROWS - 1))).unwrap();//move cursor to last first column, last row
        print("+");
        for x in 1..COLUMNS {
            print!("-");
        }
        execute!(stdout(), MoveTo((COLUMNS - 1), (ROWS - 1))).unwrap();//moves cursor to last column/row
        print!("+");
    }
}