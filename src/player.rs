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

    pub async fn use_key(&mut self) -> Option<Cords> {
        if let Some(Cords(x,y)) = self.current_position {
            match self.key_reader.read_key().await {
                Some(Key::ArrowLeft) => {
                    if y > 0 {
                        self.move_to(Cords(x, y - 1));
                    }
                }
                Some(Key::ArrowRight) => {
                    if y < COLUMNS - 1 {
                        self.move_to(Cords(x, y +1));
                    }
                }
                Some(Key::ArrowUp) => {
                    return Some(Cords(x -1, y));
                }
                Some(Key::CtrlC) => exit(0),
                _ => {}
            };
        }
        None
    }

    pub fn move_to(&mut self, new_position: Cords) {
        self.current_position = Some(new_position);
    }

    pub fn handle_collision(&mut self) -> Option<u8> {
        self.lives -= 1;
        if self.lives == 0 {
            None
        } else {
            self.current_position = None;
            Some(self.lives)
        }
    }

    pub fn respawn(&mut self, can_respawn: bool) {
        if self.current_position.is_none() && self.death_timer.tick() {
            if can_respawn {
                self.move_to(self.start_position);
            }
        }
    }
}