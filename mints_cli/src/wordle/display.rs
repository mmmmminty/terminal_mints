use std::{collections::HashMap, io::Write, time::Duration};

use crate::wordle::Wordle;
use colored::Colorize;
use terminal_size::{terminal_size, Width};

use mints_lib::*;

pub struct Display;

pub enum DisplayType {
    Start,
    GameBoard,
    Victory,
    Failure,
}

impl Display {
    pub fn display(display: DisplayType, game: &Wordle) {
        // Clear terminal
        print!("{}[2J", 27 as char);
        newln!();

        let term_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80 // Default width in case terminal size can't be determined
        };

        let header_text = match display {
            DisplayType::Start => " Wordle! ".to_string(),
            DisplayType::GameBoard => format!(" Round {} ", game.turn + 1),
            DisplayType::Victory => " You Won! ".to_string(),
            DisplayType::Failure => format!(" The word was {}! ", game.answer),
        };

        // Calculate padding
        let text_width = header_text.len() + (game.max_letters as usize * 2); // Adding 10 for the side indicators ("== ", " ==")
        let padding_length = term_width / 2 - text_width / 2;
        let padding = "=".repeat(padding_length);

        // Print header with padding
        println!("{}{}{}", padding, header_text.bold(), padding);

        for index in 0..game.max_guesses {
            Self::print_row(game, index);
        }

        newln!();
    }

    fn print_row(game: &Wordle, index: i32) {
        let term_width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80 // Default width in case terminal size can't be determined
        };

        let padding_length = term_width / 2 - game.max_letters as usize * 3;
        let padding = "=".repeat(padding_length);

        print!("{}", padding);

        let sides = if index == game.turn {
            (">> ", " <<")
        } else {
            ("== ", " ==")
        };

        print!("{}", sides.0);

        if let Some(guess) = game
            .guesses
            .get(&index)
            .expect("Guess does not exist at turn index")
        {
            let mut answer_map = HashMap::new();
            // First, count all letters in the answer
            for c in game.answer.chars() {
                *answer_map.entry(c).or_insert(0) += 1;
            }

            // First pass: Check for correct position (green)
            let guess_chars: Vec<char> = guess.chars().collect();
            let mut correctness = vec![false; guess.len()];

            for (i, &c) in guess_chars.iter().enumerate() {
                if game.answer.chars().nth(i) == Some(c) {
                    correctness[i] = true;
                    *answer_map.get_mut(&c).unwrap() -= 1;
                }
            }

            // Second pass: Check for correct letters in wrong positions (yellow)
            for (i, &c) in guess_chars.iter().enumerate() {
                if correctness[i] {
                    print!(" {} ", c.to_string().green());
                } else if answer_map.get(&c).unwrap_or(&0) > &0 {
                    print!(" {} ", c.to_string().yellow());
                    *answer_map.get_mut(&c).unwrap() -= 1;
                } else {
                    print!(" {} ", c.to_string());
                }

                if index == game.turn - 1 {
                    std::thread::sleep(Duration::from_millis(300));
                    std::io::stdout().flush().expect("Failed to flush stdout");
                }
            }
        } else {
            for _ in 0..game.max_letters {
                print!(" _ ");
            }
        }

        print!("{}", sides.1);
        print!("{}", padding);
        newln!();
    }
}
