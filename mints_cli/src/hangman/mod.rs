use std::{collections::HashMap, time::Instant, error::Error};

use mints_lib::*;

pub const HANGMAN_WORD_SIZE: i32 = 7;

#[derive(Clone)]
pub struct Hangman {
    pub turn: i32,
    pub guesses: HashMap<char, bool>,
    pub incorrect: Vec<char>,
    pub answer: String,
    pub time_started: Instant,
    pub difficulty: Difficulty,
}

impl Game for Hangman {
    fn new(args: &Args) -> Self {
        let words = load_word_list(HANGMAN_WORD_SIZE, &args.difficulty);
        let answer = choose_random_word(&words);

        let mut guesses = HashMap::new();
        answer.chars().for_each(|c| { guesses.insert(c, false); });

        Hangman {
            turn: 0,
            guesses,
            incorrect: Vec::new(),
            answer,
            time_started: std::time::Instant::now(),
            difficulty: args.difficulty.clone(),
        }
    }

    fn start(&mut self) {
        //Display::display(DisplayType::Start, &self);
        println!("Answer: {}", self.answer);
        println!("Hint: {}", hint(&self.answer));
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        todo!()
    }

    fn finish(self) {
        println!("The word was {}!", self.answer);
        drop(self);
    }
}
