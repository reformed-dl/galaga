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

    pub fn display_board(&self) {
        execute!(stdout(), MoveTo(0, 0)).unwrap();
    }
}