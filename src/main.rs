use hangman::Game;

fn main() {
    let mut game_instance = Game::instance();

    game_instance.run();
}

