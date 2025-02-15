//Game Board Dimensions
const BOARD_WIDTH: u32 = 20;
const BOARD_HEIGHT: u32 = 20;

//Player Value
const STARTING_LIVES: u8 = 5;
const DISPLAY_CHARACTER: char = '$';

//HashMap from Collections
std::collections::HashMap::

//Player Struct
#[derive(Debug)]
pub struct Player {
    display_character: char,
    lives: u8,
    current_position: (u8, u8),
}

//Player Implementation
impl Player {
    fn new(start_position: (u8, u8)) -> Self {
        Player {
            display_character: DISPLAY_CHARACTER,
            lives: STARTING_LIVES,
            current_position: start_position,
        }
    }
}
//GameState Struct
#[derive(Debug)]
pub struct GameState {
    game_board: HashMap<(u8, u8), char>,
    player: &Player,
}


fn main() {
    println!("Game Board Dimensions {} x {}", BOARD_WIDTH, BOARD_HEIGHT);
    println!("Player Starting Lives: {}", STARTING_LIVES);
    println!("Player Character: {}", DISPLAY_CHARACTER);
    let player = Player {
        display_character: '$',
        lives: STARTING_LIVES,
        current_position: (5, 5),
    };
    println!("Player Info: {:?}", player);

    let player = Player::new((5,5));
    println!("Player Created: {:?}", player);

    let mut game_board = HashMap::new();
    game_board.insert((5,5), "$");
    let player = Player::new((5,5));
    let game_state = GameState{game_board, player: &player};
    println!("Game State: {:?}", game_state);
}
