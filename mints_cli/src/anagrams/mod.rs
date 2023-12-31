use std::{time::{Instant, Duration}, error::Error, io::Write, thread::{sleep, self}, collections::HashMap};

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
        Anagrams {
            current_level: 0,
            max_guesses: None,
            current_guesses: 0,
            anagram: None,
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
        clear!();

        self.display_big_scramble();
    }

    fn do_loop(&mut self) -> Result<i32, Box<dyn Error>> {
        if self.anagram.is_none() {
            println!("Checking in 2s..");
            sleep!(2000);
            return Ok(GAME_RESTART);
        }

        todo!()
    }

    fn finish(self) {
        todo!()
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
        newln!();
        print!("Welcome to");

        for _ in 0..3 {
            print!(".");
            flush!();
            sleep!(700);
        }

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

    fn display_big_scramble(&self) {
        let scramble = self.get_scramble().to_ascii_uppercase();
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

        print!("{}", terminal_fonts::to_block_string(scramble.as_str()).yellow());
        newln!();
    }

    fn get_scramble(&self) -> String {
        self
            .anagram
            .clone()
            .expect("Anagram not chosen!")
            .scramble
    }

    fn get_words(&self) -> HashMap<usize, Vec<String>> {
        self
            .anagram
            .clone()
            .expect("Anagram not chosen!")
            .words
    }
}