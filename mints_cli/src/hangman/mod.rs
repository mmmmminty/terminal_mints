use std::{time::{Instant, Duration}, error::Error};

use colored::Colorize;
use terminal_size::{terminal_size, Width};

use mints_lib::*;

const HANGMAN_WORD_SIZE: i32 = 7;
const HANGMAN_GUESS_SIZE: i32 = 7;

const ASCII_0: &str = include_str!("./ascii/0.txt");
const ASCII_1: &str = include_str!("./ascii/1.txt");
const ASCII_2: &str = include_str!("./ascii/2.txt");
const ASCII_3: &str = include_str!("./ascii/3.txt");
const ASCII_4: &str = include_str!("./ascii/4.txt");
const ASCII_5: &str = include_str!("./ascii/5.txt");
const ASCII_6: &str = include_str!("./ascii/6.txt");
const ASCII_WIN: &str = include_str!("./ascii/win.txt");
const ASCII_LOSE: &str = include_str!("./ascii/lose.txt");

#[derive(Clone)]
pub struct Hangman {
    pub turn: i32,
    pub correct: Vec<char>,
    pub incorrect: Vec<char>,
    pub answer: String,
    pub time_started: Instant,
    pub difficulty: Difficulty,
    pub ascii: Vec<String>
}

impl Game for Hangman {
    fn new(args: &Args) -> Self {
        let words = load_word_list(HANGMAN_WORD_SIZE, &args.difficulty);
        let answer = choose_random_word(&words);

        Hangman {
            turn: 0,
            correct: Vec::new(),
            incorrect: Vec::new(),
            answer,
            time_started: std::time::Instant::now(),
            difficulty: args.difficulty.clone(),
            ascii: vec![ASCII_0, ASCII_1, ASCII_2, ASCII_3, ASCII_4, ASCII_5, ASCII_6]
                .iter()
                .map(|s| s.to_string())
                .collect()
        }
    }

    fn start(&mut self) {

    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        self.display();
        self.incorrect.push('c');
        std::thread::sleep(Duration::from_secs(1));
        Ok(GAME_ONGOING)
    }

    fn finish(self) {
        println!("The word was {}!", self.answer);
        drop(self);
    }
}

impl Hangman {
    fn display(&self) {
        // Clear terminal
        print!("{}[2J", 27 as char);
        println!();

        let term_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80 // Default width in case terminal size can't be determined
        };

        let header_text = match self.turn {
            0 => " Hangman! ".to_string(),
            _ => format!(" Round {} ", self.turn + 1)
        };

        // Calculate padding
        let text_width = header_text.len() + (HANGMAN_WORD_SIZE as usize * 2); // Adding 10 for the side indicators ("== ", " ==")
        let padding_length = term_width / 2 - text_width / 2;
        let padding = "=".repeat(padding_length);

        // Print header with padding
        println!("{}{}{}", padding, header_text.bold(), padding);
        println!();

        // Print ascii art
        print!("{}", self.ascii[self.incorrect.len()]);

    }
}
