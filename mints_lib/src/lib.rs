use std::error::Error;
use clap::{ValueEnum, Parser, value_parser};
use rand::seq::SliceRandom;

/// Used to signify to the `mint_cli` to continue the game loop.
pub const GAME_ONGOING: i32 = 31;

/// Used to signify to the `mint_cli`to restart the game loop.
pub const GAME_RESTART: i32 = 32;

/// Used to signify to the `mint_cli` to break the game loop.
pub const GAME_OVER: i32 = 33;

// These are all added as const strings as to be embedded in the binary.
// Any word lists to be added need to be embedded in the binary to be read
// by the read_word_list function.
const WORDS_4E: &str = include_str!("../word_lists/4E.txt");
const WORDS_4M: &str = include_str!("../word_lists/4M.txt");
const WORDS_4H: &str = include_str!("../word_lists/4H.txt");
const WORDS_5E: &str = include_str!("../word_lists/5E.txt");
const WORDS_5M: &str = include_str!("../word_lists/5M.txt");
const WORDS_5H: &str = include_str!("../word_lists/5H.txt");
const WORDS_6E: &str = include_str!("../word_lists/6E.txt");
const WORDS_6M: &str = include_str!("../word_lists/6M.txt");
const WORDS_6H: &str = include_str!("../word_lists/6H.txt");
const WORDS_7E: &str = include_str!("../word_lists/7E.txt");
const WORDS_7M: &str = include_str!("../word_lists/7M.txt");
const WORDS_7H: &str = include_str!("../word_lists/7H.txt");
const WORDS_8E: &str = include_str!("../word_lists/8E.txt");
const WORDS_8M: &str = include_str!("../word_lists/8M.txt");
const WORDS_8H: &str = include_str!("../word_lists/8H.txt");

/// Default methods for a terminal-based game.
pub trait Game {
    /// Used to transform the arguments, if any, into the game object.
    fn new(args: &Args) -> Self;

    /// Starts the game. This is called **once** at the beginning by the 
    /// `mint_cli`.
    fn start(&mut self);

    /// What should be done in a loop. This is called in a loop by the
    /// `mint_cli` until an error occurs, or a code other than `GAME_ONGOING`
    /// is returned. 
    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>>;

    /// Finishes the game. Anything to be cleaned up/done **once** at the end
    /// of the game loop should be done here.
    fn finish(self);
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Mints {
    Wordle,
    Hangman
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(help = "The game to play (e.g., 'wordle', 'hangman')")]
    pub game: Mints,

    #[arg(short = 'g', long = "guesses", default_value_t = 6, value_parser = value_parser!(i32).range(3..=9))]
    pub guesses: i32,

    #[arg(short = 'l', long = "letters", default_value_t = 5, value_parser = value_parser!(i32).range(4..=8))]
    pub letters: i32,

    #[arg(short = 'd', long = "difficulty", default_value_t = Difficulty::Easy)]
    #[clap(value_enum)]
    pub difficulty: Difficulty,
}

pub fn choose_random_word(words: &[String]) -> String {
    let mut rng = rand::thread_rng();
    words.choose(&mut rng)
        .cloned()
        .expect("Failed to pick random word")
}

pub fn load_word_list(letters: i32, diff: &Difficulty) -> Vec<String> {
    let txt = match letters {
        4 => {
            match diff {
                Difficulty::Easy => WORDS_4E,
                Difficulty::Medium => WORDS_4M,
                Difficulty::Hard => WORDS_4H,
            }
        },
        5 => {
            match diff {
                Difficulty::Easy => WORDS_5E,
                Difficulty::Medium => WORDS_5M,
                Difficulty::Hard => WORDS_5H,
            }
        },
        6 => {
            match diff {
                Difficulty::Easy => WORDS_6E,
                Difficulty::Medium => WORDS_6M,
                Difficulty::Hard => WORDS_6H,
            }
        },
        7 => {
            match diff {
                Difficulty::Easy => WORDS_7E,
                Difficulty::Medium => WORDS_7M,
                Difficulty::Hard => WORDS_7H,
            }
        },
        8 => {
            match diff {
                Difficulty::Easy => WORDS_8E,
                Difficulty::Medium => WORDS_8M,
                Difficulty::Hard => WORDS_8H,
            }
        },
        _ => unreachable!()
    };

    let mut words = Vec::new();

    for line in txt.lines() {
        for word in line.split_ascii_whitespace() {
            words.push(word.to_ascii_uppercase());
        }
    }
    
    words
}

pub fn word_exists(letters: i32, word: String) -> bool {
    load_word_list(letters, &Difficulty::Easy).contains(&word)
    || load_word_list(letters, &Difficulty::Medium).contains(&word)
    || load_word_list(letters, &Difficulty::Hard).contains(&word)
}