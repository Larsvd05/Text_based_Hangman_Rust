mod common;
mod user_interaction;
mod game;


fn main() {
    
    let mut game = game::HangmanGame::setup();

    game.play();
}
