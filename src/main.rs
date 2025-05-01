mod game_state;
mod structs;
mod player;

use game_state::GameState;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut game = GameState::new();
    game.start_game().await?;
    Ok(())
}
