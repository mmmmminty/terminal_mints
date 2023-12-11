use clap::Parser;
use mints_lib::*;

// Games:
mod hangman;
mod wordle;

fn main() {
    let args = Args::parse();

    match args.game {
        Mints::Wordle => run_game(wordle::Wordle::new(&args), &args),
        Mints::Hangman => run_game(hangman::Hangman::new(&args), &args),
    }
}

fn run_game<G: Game>(mut game: G, args: &Args) {
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
                run_game(wordle::Wordle::new(args), args);
                break;
            }
            _ => break,
        }
    }

    game.finish();
    std::process::exit(code);
}
