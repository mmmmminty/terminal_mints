use std::{time::Instant, error::Error, io::{Write, BufReader, BufRead}, thread::{self}, collections::HashMap};

use colored::Colorize;
use mints_lib::*;
use terminal_size::terminal_size;
use self::{anagram::Anagram, utils::AnagramParams};

mod anagram;
mod display;
mod utils;

/// This is the size of the scramble the user will need to solve. This is also
/// consequently the maximum size of words in the game.
pub const ANAGRAM_SCRAMBLE_SIZE: usize = 8;

/// This list contains more common words used as to avoid random guessing of words that
/// don't seem like exist. When this is paired with a entry min of 5 and max of 8, you 
/// get 300 different anagrams. 
pub const ANAGRAM_WORD_LIST: &str = include_str!("./anagram_words.txt");

/// This is the minimum terminal width required to play Anagrams (as a result of the big text).
const MIN_TERM_WIDTH: usize = 75;

#[derive(Clone)]
pub struct Anagrams {
    pub current_level: i32,
    pub max_guesses: Option<i32>,
    pub current_guesses: i32,
    pub anagram: Option<Anagram>,
    pub answers: HashMap<usize, Vec<String>>,
    pub time_started: Instant,

    /// The difficulty in this case is not the words themselves, but the
    /// amount of entries the player is required to solve before moving to the
    /// next level.
    pub difficulty: Difficulty,

    pub params: AnagramParams
}

impl Game for Anagrams {
    fn new(args: &Args) -> Self {
        let params = AnagramParams::new(&args.difficulty);
        let mut answers = HashMap::new();

        for i in params.letter_range.clone() {
            answers.insert(i, Vec::<String>::new());
        }

        Anagrams {
            current_level: 0,
            max_guesses: None,
            current_guesses: 0,
            anagram: None,
            answers,
            time_started: Instant::now(),
            difficulty: args.difficulty.clone(),
            params
        }
    }

    fn start(&mut self) {
        // let list = WORDS_MASTER.to_string();
        let list = ANAGRAM_WORD_LIST.to_string();
        // Uncomment to shadow and filter for only words defined by webster
        // let mut list = "".to_string();
        // for word in WORDS_MASTER.split_ascii_whitespace() {
        //     if webster::dictionary(word).is_some() {
        //         list.push_str(format!(" {word}").as_str());
        //     }
        // }

        let term_width = if let Some((terminal_size::Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80 // Default width in case terminal size can't be determined
        };     

        if term_width < MIN_TERM_WIDTH {
            println!("Increase your terminal size goof! (Curr: {term_width}, Min: {MIN_TERM_WIDTH})");
            return;
        }

        // Hopefully this spends less than 5s picking a scramble.
        thread::scope(|s| {
            s.spawn(|| { self.anagram = Some(Anagram::new(&list, ANAGRAM_SCRAMBLE_SIZE, &self.params)) });
            Self::loading_screen();
        });

        self.display_big_scramble(false);
        self.display(None);
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        if self.anagram.is_none() {
            println!("Checking in 2s..");
            sleep!(2000);
            return Ok(GAME_RESTART);
        }

        let mut input = BufReader::new(std::io::stdin()).lines();
        let guess = match input.next() {
            Some(s) => s.unwrap().to_ascii_uppercase(),
            None => return Ok(GAME_OVER),
        };

        if !guess.chars().all(|c| c.is_ascii_alphabetic()) {
            println!("No special characters allowed silly!");

        } else if !self.params.letter_range.contains(&guess.len()) {
            println!("No {}-letter words needed goof!", guess.len());

        } else {
            if !self.anagram.as_ref().unwrap().valid_word(&guess.to_ascii_lowercase()) {
                self.display_big_scramble(true);
                self.display(Some(guess));
            } else {
                if !self.insert_entry(&guess) {
                    println!("No more {}-letter words needed", guess.len());
                } else {
                    self.display_big_scramble(true);
                    self.display(None);
                }
            }
        }
        
        if self.check_win() {
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
        drop(self);
    }
}

impl Anagrams {
    /// This gives the game about 5s to pick a scramble, which should be enough time.
    /// This is not implemented with any actual asynchronous code, it's just a filler.
    /// You must clear the screen when you're ready!
    fn loading_screen() {
        let middle = if let Some((_, terminal_size::Height(h))) = terminal_size() {
            (h as usize / 2) - 3
        } else {
            0 // Default width in case terminal size can't be determined
        };  

        let header = "ANAGRAMS";
        // newln!();
        // print!("Welcome to");

        // for _ in 0..3 {
        //     print!(".");
        //     flush!();
        //     sleep!(700);
        // }

        clear!();

        for (i, _) in header.chars().enumerate() {
            print!("{}", terminal_fonts::to_block_string(&header[0..=i]));
            newln!(middle);
            flush!();
            
            if i == header.len() - 1 {
                sleep!(800);
            } else {
                sleep!(300);
                clear!();
            }
            newln!();
        }
    }

    fn display_big_scramble(&self, skip_animation: bool) {
        let scramble = self.get_scramble().to_ascii_uppercase();
        clear!();

        if !skip_animation {
            for (i, _) in scramble.chars().enumerate() {
                print!("{}", terminal_fonts::to_block_string(&scramble[0..=i]));
                newln!();
                flush!();
                
                if i == scramble.len() - 1 {
                    sleep!(500);
                } else {
                    sleep!(100);
                }
                
                clear!();
                newln!();
            }
        }

        print!("{}", terminal_fonts::to_block_string(scramble.as_str()).yellow());
        newln!(3);
    }

    fn display(&self, wrong: Option<String>) {
        let sections: Vec<usize> = self.params.letter_range.clone().into_iter().collect();

        for section in sections {
            self.print_section(section, &wrong);
        }

        if self.check_win() {
            newln!();
            println!("You won!");
        }
    }

    fn print_section(&self, section: usize, wrong: &Option<String>) {
        let section_width = terminal_fonts::to_block_string(self.get_scramble().as_str())
            .split_terminator('\n')
            .into_iter()
            .next()
            .unwrap()
            .len() / 2 - 8;

        let header = format!(" {section}-letter words ");
        let padding_length = (section_width - header.len()) / 2;
        let padding = "=".repeat(padding_length);

        println!("{}", "=".repeat(section_width));
        println!("{}{}{}", padding, header.bold(), padding);

        let row_count = self.params.entry_min(section);
        for row in 0..row_count {
            self.print_row(row, section_width, section, wrong);
        }

        println!("{}", "=".repeat(section_width));
        newln!(2);
    }

    fn print_row(&self, row: usize, width: usize, section: usize, wrong: &Option<String>) {
        let placeholder = format!(" {}", "_ ".repeat(section));
        let padding_width = ((width - placeholder.len()) / 2) - 2;
        let left = format!("{}>>", "=".repeat(padding_width));
        let right = format!("<<{}", "=".repeat(padding_width + 1));

        match self.answers.get(&section).unwrap().get(row) {
            Some(answer) => {
                print!("{} ", left);
                    for c in answer.chars() {
                        print!("{} ", c.to_ascii_uppercase().to_string().green());
                        flush!();

                        if self.answers.get(&section).unwrap().get(row + 1).is_none()
                        && section == answer.len() {
                            sleep!(100);
                        }
                    }
                println!("{}", right);
            },
            None => {
                if let Some(guess) = wrong {
                    if section == guess.len() 
                    && (self.answers.get(&section).unwrap().get(row - 1).is_some() 
                    || (self.answers.get(&section).unwrap().get(row).is_none() 
                    && row == 0)) {
                        print!("{} ", left);
                        for c in guess.chars() {
                            print!("{} ", c.to_ascii_uppercase().to_string().red());
                            flush!();
                            sleep!(50);
                        }
                        print!("{}", right);
                        sleep!(500);
                    }
                }

                println!("\r\x1B[K{}{}{}", left, placeholder, right);
                flush!();
            },
        }
    }

    fn insert_entry(&mut self, guess: &String) -> bool {
        let entry_limit = self.params.entry_min(guess.len());
        let entry_count = self
            .answers
            .get(&guess.len())
            .unwrap()
            .len();

        let res = entry_count < entry_limit;

        if res {
            self.answers.entry(guess.len()).and_modify(|e| {
                if !e.contains(guess) {
                    e.push(guess.to_owned()); 
                }
            });
        }

        res
    }

    fn check_win(&self) -> bool {
        for i in self.params.letter_range.clone() {
            let entries_required = self.params.entry_min(i);
            if self.answers.get(&i).unwrap().len() != entries_required {
                return false;
            }
        }
        true
    }

    fn get_scramble(&self) -> String {
        self
            .anagram
            .clone()
            .expect("Anagram not chosen!")
            .scramble
    }
}