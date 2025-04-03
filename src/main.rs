//Game Board Dimensions
const BOARD_WIDTH: u32 = 20;
const BOARD_HEIGHT: u32 = 20;

//Player Value
const STARTING_LIVES: u8 = 5; 
const DISPLAY_CHARACTER: char = '$';

//Player Struct
#[derive(Debug)]
struct Player {
    display_character: char,
    lives: u8,
    current_position: (u8, u8),
}

fn main() {
    println!("Gameboard Dimension are: {} x {}", BOARD_HEIGHT, BOARD_WIDTH);
    println!("Player Starting Lives: {}, Symbol: {}", STARTING_LIVES, DISPLAY_CHARACTER);

    let player = Player {
        display_character: DISPLAY_CHARACTER,
        lives: STARTING_LIVES,
        current_position: (5, 5),
    };
    println!("Player Stats: {:?}", player);
}