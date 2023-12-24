use std::{
    collections::HashSet,
    error::Error,
    io::{BufRead, BufReader, Write},
    time::{Duration, Instant},
};

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
    pub ascii: Vec<String>,
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
            ascii: vec![
                ASCII_0, ASCII_1, ASCII_2, ASCII_3, ASCII_4, ASCII_5, ASCII_6,
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }

    fn start(&mut self) {
        self.display(None);
        // println!("Answer: {}", self.answer);
        // println!("Hint: {}", hint(&self.answer));
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut input = BufReader::new(std::io::stdin()).lines();
        let guess = match input.next() {
            Some(s) => s.unwrap().to_ascii_uppercase(),
            None => return Ok(GAME_OVER),
        };
        let letter = guess.chars().nth(0).unwrap_or(';');

        // Handle command and early return
        if guess.starts_with('!') {
            return Ok(self.handle_commands(&guess.to_ascii_lowercase()));
        }

        // Correct amount of letters
        if guess.len() != 1 {
            println!("Guess one letter at a time!");

        // Is a ascii alphabetical
        } else if !letter.is_ascii_alphabetic() {
            println!("{letter} is not a letter silly!");

        // Already guessed
        } else if self.incorrect.contains(&letter) || self.correct.contains(&letter) {
            println!("You've guesses '{letter}' already!");

        // Otherwise go to next turn
        } else {
            if self.answer.contains(letter) {
                self.correct.push(letter);
            } else {
                self.incorrect.push(letter);
            }

            if self.check_win() {
                self.display(Some(true));
            } else if self.incorrect.len() == HANGMAN_GUESS_SIZE as usize {
                self.display(Some(false));
            } else {
                self.turn += 1;
                self.display(None);
                return Ok(GAME_ONGOING);
            }

            println!("Play again? (y/n)");
            return match input.next() {
                Some(s) => match s.unwrap().as_str() {
                    "Y" | "y" => Ok(GAME_RESTART),
                    _ => Ok(GAME_OVER),
                },
                None => Ok(GAME_OVER),
            };
        }

        Ok(GAME_ONGOING)
    }

    fn finish(self) {
        println!("The word was {}!", self.answer);
        drop(self);
    }
}

impl Hangman {
    fn display(&self, win: Option<bool>) {
        // Clear terminal
        print!("{}[2J", 27 as char);
        newln!();

        let term_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80 // Default width in case terminal size can't be determined
        };

        let header_text = match self.turn {
            0 => " Hangman! ".to_string(),
            _ => {
                if win.is_none() {
                    format!(" Round {} ", self.turn + 1)
                } else if win.unwrap() {
                    " You won! ".to_string()
                } else {
                    format!(" The word was {}! ", self.answer)
                }
            }
        };

        // Calculate padding
        let text_width = header_text.len() + (HANGMAN_WORD_SIZE as usize * 2); // Adding 10 for the side indicators ("== ", " ==")
        let padding_length = term_width / 2 - text_width / 2;
        let padding = "=".repeat(padding_length);

        // Print header with padding
        println!("{}{}{}", padding, header_text.bold(), padding);
        newln!();

        // Print ascii art
        if win.is_none() {
            print!("{}", self.ascii[self.incorrect.len()]);
        } else if win.unwrap() {
            print!("{}", ASCII_WIN);
        } else {
            print!("{}", ASCII_LOSE);
        }
        newln!(2);

        let bar = "=".repeat((padding_length * 2) + header_text.len());
        println!("{bar}");
        newln!();

        // Print correct words
        for c in self.answer.chars() {
            if self.correct.contains(&c) {
                print!(" {} ", c.to_string().green().bold());
            } else {
                print!(" {} ", "_".to_string().bold());
            }
        }

        newln!(2);
        println!("{bar}");
        newln!();

        for c in &self.incorrect {
            print!(" {} ", c.to_string().red().bold());
        }

        newln!(2);
    }

    fn check_win(&self) -> bool {
        let answer: HashSet<char> = self.answer.chars().collect();
        let correct: HashSet<char> = self.correct.clone().into_iter().collect();

        answer == correct
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
                std::io::stdout().flush().expect("Failed to flush stdout");
                std::thread::sleep(Duration::from_secs(1));
                print!("2.. ");
                std::io::stdout().flush().expect("Failed to flush stdout");
                std::thread::sleep(Duration::from_secs(1));
                print!("1.. ");
                std::io::stdout().flush().expect("Failed to flush stdout");
                std::thread::sleep(Duration::from_secs(1));

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
