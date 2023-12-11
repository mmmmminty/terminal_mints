use mints_lib::*;

use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader},
    time::Instant,
};

mod display;
use crate::wordle::display::*;

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
    fn new(args: &Args) -> Self {
        let words = load_word_list(args.letters, &args.difficulty);

        let mut map: HashMap<i32, Option<String>> = HashMap::new();
        for i in 0..args.guesses {
            map.insert(i, None);
        }

        Wordle {
            turn: 0,
            answer: choose_random_word(&words),
            guesses: map,
            words,
            time_started: std::time::Instant::now(),
            max_guesses: args.guesses,
            max_letters: args.letters,
            difficulty: args.difficulty.clone(),
        }
    }

    fn start(&mut self) {
        Display::display(DisplayType::Start, &self);
        //println!("Answer: {}", self.answer);
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut input = BufReader::new(std::io::stdin()).lines();
        let guess = match input.next() {
            Some(s) => s.unwrap().to_ascii_uppercase(),
            None => return Ok(GAME_OVER),
        };

        // Correct amount of letters
        if guess.len() != self.max_letters as usize {
            println!("{guess} is not a {}-letter word!", self.max_letters);

        // Is an actual word
        } else if !self.words.contains(&guess) {
            println!("{} is not a word silly!", guess);

        // Already guessed
        } else if self
            .guesses
            .values()
            .any(|g| g
                    .as_ref()
                    .is_some_and(|s| s.contains(&guess))
                )
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
}
