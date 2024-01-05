use mints_lib::*;

use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Write},
    time::Instant,
};

use colored::Colorize;

mod display;
use crate::wordle::display::*;

/// The default amount of guesses if not specified, or incorrect.
const DEFAULT_GUESS_SIZE: i32 = 6;

/// The default word size if not specified, or incorrect.
const DEFAULT_WORD_SIZE: i32 = 5;

#[derive(Clone)]
pub struct Wordle {
    pub turn: i32,
    pub words: Vec<String>,
    pub guesses: HashMap<i32, Option<String>>,
    pub answer: String,
    pub time_started: Instant,
    pub max_guesses: i32,
    pub max_letters: i32,
    pub difficulty: Difficulty,
}

impl Game for Wordle {
    fn new(mut args: Args) -> Self {
        Wordle::parse_args(&mut args);
        let words = load_word_list(args.letters.unwrap(), &args.difficulty.as_ref().unwrap());

        let mut map: HashMap<i32, Option<String>> = HashMap::new();
        for i in 0..args.guesses.unwrap() {
            map.insert(i, None);
        }

        Wordle {
            turn: 0,
            answer: choose_random_word(&words),
            guesses: map,
            words,
            time_started: std::time::Instant::now(),
            max_guesses: args.guesses.unwrap(),
            max_letters: args.letters.unwrap(),
            difficulty: args.difficulty.unwrap(),
        }
    }

    fn start(&mut self) {
        titled_loading_screen("wordle", "white", 800);
        Display::display(DisplayType::Start, &self);
        // println!("Answer: {}", self.answer);
        // println!("Hint: {}", hint(&self.answer));
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut input = BufReader::new(std::io::stdin()).lines();
        let guess = match input.next() {
            Some(s) => s.unwrap().to_ascii_uppercase(),
            None => return Ok(GAME_OVER),
        };

        // Handle command and early return
        if guess.starts_with('!') {
            return Ok(self.handle_commands(&guess.to_ascii_lowercase()));
        }

        // Correct amount of letters
        if guess.len() != self.max_letters as usize {
            println!("{guess} is not a {}-letter word!", self.max_letters);

        // Is an actual word
        } else if !word_exists(self.max_letters, &guess) {
            println!("{} is not a word silly!", guess);

        // Already guessed
        } else if self
            .guesses
            .values()
            .any(|g| g.as_ref().is_some_and(|s| s.contains(&guess)))
        {
            println!("You've guessed {guess} already!");

        // Otherwise register guess
        } else {
            Display::display(self.guess(&guess), &self);
            if self.turn == self.max_guesses || self.answer.contains(&guess) {
                println!("Play again? (y/n)");

                return match input.next() {
                    Some(s) => match s.unwrap().as_str() {
                        "Y" | "y" => Ok(GAME_RESTART),
                        _ => Ok(GAME_OVER),
                    },
                    None => Ok(GAME_OVER),
                };
            }
        }

        Ok(GAME_ONGOING)
    }

    fn finish(self) {
        println!("The word was {}!", self.answer);
        drop(self);
    }
}

impl Wordle {
    fn parse_args(args: &mut Args) {
        if args.guesses.is_none() {
            args.guesses = Some(DEFAULT_GUESS_SIZE);
        } else if args.guesses.is_some_and(|g| g < 3 || g > 9) {
            warning!("Guesses must be between 3 & 9!");
            args.guesses = Some(DEFAULT_GUESS_SIZE);
        }

        if args.letters.is_none() {
            args.letters = Some(DEFAULT_WORD_SIZE);
        } else if args.letters.is_some_and(|l| l < 4 || l > 8) {
            warning!("Letters must be between 4 & 8!");
            args.letters = Some(DEFAULT_WORD_SIZE);
        }

        if args.difficulty.is_none() {
            args.difficulty = Some(Difficulty::Easy);
        }
    }

    fn guess(&mut self, guess: &String) -> DisplayType {
        self.guesses.insert(self.turn, Some(guess.clone()));
        self.turn += 1;

        if self.answer.contains(guess) {
            DisplayType::Victory
        } else if self.turn == self.max_guesses {
            DisplayType::Failure
        } else {
            DisplayType::GameBoard
        }
    }

    fn handle_commands(&mut self, cmd: &String) -> i32 {
        match cmd.as_str() {
            "!hint" | "!h" => {
                println!("Hint: {}", hint(&self.answer));
                GAME_ONGOING
            }
            "!restart" | "!next" | "!reset" | "!r" => {
                println!("The word was {}!", self.answer);

                print!("Restarting in 3.. ");
                flush!();
                sleep!(1000);
                print!("2.. ");
                flush!();
                sleep!(1000);
                print!("1.. ");
                flush!();
                sleep!(1000);

                GAME_RESTART
            }
            "!quit" | "!leave" | "!exit" | "!q" => GAME_OVER,
            _ => {
                println!("Unknown command!");
                GAME_ONGOING
            }
        }
    }
}
