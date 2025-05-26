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
    pub key_reader: KeyReader,//KeyReader Struct for key input   
}

impl Player {
    pub fn new() -> Self {
        let start_position = Cords(ROWS - 2, COLUMNS/2);//positions player at bottom center of the screen
        Player {
            display_char: '^',//player appears as upward arrow
            lives: 4, //player starts with 4 lives
            current_position: Some(start_position),//player begins game at start position
            start_position, //start position is hard coded 
            death_timer: Timer::new(200), //respawn delay timer (interval)
            key_reader: KeyReader::new(), //initialize KeyReader struct for storing key inputs
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

//jh is the name of the field that hold a handle to an asynchronous task responsible for retrieving keys
//It is wrapped in an Option because sometimes there is a task running Some() and other times there isnt, None
//tokio::task::JoinHandle<T> is the type returned when spawning an async task using tokio::spawn()
//T in this case is Key, which is a value from console::Key enum (which represents a key press by the user)
//This can spawn a background task that "listens" for key presses, and we can "store the handle" so we can later, cancel, await, check if done, etc.
pub struct KeyReader {
    pub jh: Option<tokio::task::JoinHandle<Key>>,
}

impl KeyReader {
    //here we are implementing a constructor method for a new instance of the KeyReader Struct
    //as idicated by the KeyReader struct, jh has a type of tokio::task::JoinHandle<Key>>
    //This type will spawn a background async task using tokio::spawn() and run the await_key_press method we haven't coded yet
    //Store the handle to that task (JoinHandle<Key>) in the KeyReader struct's jh field, which is an Option Enum, Some() or None
    //We are creating a new instance of KeyReader, with a background task that listens for a key press, saves the JoinHandle in the jh field for later
    pub fn new() -> KeyReader {
        KeyReader { jh: Some(tokio::spawn(KeyReader::await_key_press())), }
    }

    //this async function has a return type of Key, from console::Key
    //we declare the variable term and set it to Term::stdout(), which creates a terminal interface object (reading/writing to standard output)
    //read_key() reads a single key press from the terminal
    //unwrap() extracts the Result enum returned by read_key()
    pub async fn await_key_press() -> Key {
        let term = Term::stdout();
        term.read_key().unwrap()
    }
}