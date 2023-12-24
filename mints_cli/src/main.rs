use clap::Parser;
use mints_lib::*;

// Games:
mod hangman;
mod wordle;

fn main() {
    let args = Args::parse();

    while match args.game {
        Mints::Wordle => run_game(wordle::Wordle::new(&args)),
        Mints::Hangman => run_game(hangman::Hangman::new(&args)),
    } {}
}

fn run_game<G: Game>(mut game: G) -> bool {
    game.start();

    let mut code;
    loop {
        code = match game.do_loop() {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Game Error: {e}");
                1
            }
        };

        match code {
            GAME_ONGOING => continue,
            GAME_OVER => break,
            GAME_RESTART => {
                return true;
            }
            _ => break,
        }
    }

    game.finish();
    std::process::exit(code);
}
