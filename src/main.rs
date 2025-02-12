//Game Board Dimensions
const BOARD_WIDTH: u32 = 20;
const BOARD_HEIGHT: u32 = 20;

//Player Value
const STARTING_LIVES: u8 = 5;
const DISPLAY_CHARACTER: char = '$';


fn main() {
    println!("Game Board Dimensions {} x {}", BOARD_WIDTH, BOARD_HEIGHT);
    println!("Player Starting Lives: {}", STARTING_LIVES);
    println!("Player Character: {}", DISPLAY_CHARACTER);
}
