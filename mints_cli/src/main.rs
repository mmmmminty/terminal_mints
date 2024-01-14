use clap::Parser;
use mints_lib::*;

// Games:
mod anagrams;
mod hangman;
mod wordle;
mod minesweeper;

fn main() {
    let args = Args::parse();

    while match &args.game {
        Mints::Wordle => run_game(wordle::Wordle::new(args.clone())),
        Mints::Hangman => run_game(hangman::Hangman::new(args.clone())),
        Mints::Anagrams => run_game(anagrams::Anagrams::new(args.clone())),
        Mints::Minesweeper => run_game(minesweeper::Minesweeper::new(args.clone()))
    } {}
}

/// The default execution cycle of a given `Game` object. This function returns
/// true if the exit code `GAME_RESTART` is received, and terminates the program
/// when `GAME_OVER` is received.
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
