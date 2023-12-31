// use std::{collections::HashMap, io::Write, time::Duration};

// use crate::wordle::Wordle;
// use colored::Colorize;
// use mints_lib::newln;
// use terminal_size::{terminal_size, Width};

// use super::Anagrams;

// pub struct Display;

// pub enum DisplayType {
//     Start,
//     Level(usize),
//     Victory,
//     Failure,
// }

// impl Display {
//     pub fn display(display: DisplayType, game: &Anagrams) {
//         // Clear terminal
//         print!("{}[2J", 27 as char);
//         newln!();

//         let term_width = if let Some((Width(w), _)) = terminal_size() {
//             w as usize
//         } else {
//             80 // Default width in case terminal size can't be determined
//         };

//         let header_text = match display {
//             DisplayType::Start => " Anagrams! ".to_string(),
//             DisplayType::Level(level) => format!(" Round {} ", game.turn + 1),
//             DisplayType::Victory => " You Won! ".to_string(),
//             DisplayType::Failure => format!(" The word was {}! ", game.answer),
//         };

//         // Calculate padding
//         let text_width = header_text.len() + (game.max_letters as usize * 2); // Adding 10 for the side indicators ("== ", " ==")
//         let padding_length = term_width / 2 - text_width / 2;
//         let padding = "=".repeat(padding_length);

//         // Print header with padding
//         println!("{}{}{}", padding, header_text.bold(), padding);

//         for index in 0..game.max_guesses {
//             Self::print_row(game, index);
//         }

//         println!();
//     }

//     fn print_row(game: &Anagrams, index: i32) {
//         let term_width = if let Some((Width(w), _)) = terminal_size() {
//             w as usize
//         } else {
//             80 // Default width in case terminal size can't be determined
//         };

//         let padding_length = term_width / 2 - game.max_letters as usize * 3;
//         let padding = "=".repeat(padding_length);

//         print!("{}", padding);

//         let sides = if index == game.turn {
//             (">> ", " <<")
//         } else {
//             ("== ", " ==")
//         };

//         print!("{}", sides.0);

//         print!("{}", sides.1);
//         print!("{}", padding);
//         println!();
//     }

//     pub fn loading_screen(seconds: usize) ->
// }
