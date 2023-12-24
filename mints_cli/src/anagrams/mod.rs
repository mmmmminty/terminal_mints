use std::time::Instant;

use mints_lib::*;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Anagrams {
    pub turn: i32,
    pub max_guesses: i32,
    pub max_letters: i32,
    pub answer: String,
    pub scrambled: String,
    pub time_started: Instant,
    pub difficulty: Difficulty
}

impl Game for Anagrams {
    fn new(args: &Args) -> Self {
        let words = load_word_list(args.letters, &args.difficulty);
        let answer = choose_random_word(&words);

        let mut chars: Vec<_> = answer.chars().collect();
        chars.shuffle(&mut rand::thread_rng());
        let scrambled = chars.into_iter().collect();

        Anagrams {
            turn: 0,
            max_guesses: args.guesses,
            max_letters: args.letters,
            answer,
            scrambled,
            time_started: Instant::now(),
            difficulty: args.difficulty.clone(),
        }
    }

    fn start(&mut self) {
        todo!()
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        todo!()
    }

    fn finish(self) {
        todo!()
    }
}