

mod game;
pub mod network;

fn main() {
    let mut game = game::Game::new();

    game.serve_forever();
}