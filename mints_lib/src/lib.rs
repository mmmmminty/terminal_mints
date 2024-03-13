// use clap::{value_parser, Parser, ValueEnum};
// use colored::Colorize;
// use rand::seq::SliceRandom;
// use std::{collections::HashMap, error::Error, io::Write};

// /// A clutter-helper to avoid repeated `println!()`. Specify the number of newlines
// /// desired or omit for a single one.
// #[macro_export]
// macro_rules! newln {
//     ($repeat:expr) => {
//         for _ in 0..$repeat {
//             println!();
//         }
//     };
//     () => {
//         println!();
//     };
// }

// /// A clutter helper to sleep for a specified amount of time (milliseconds)
// #[macro_export]
// macro_rules! sleep {
//     ($millis:expr) => {
//         std::thread::sleep(std::time::Duration::from_millis($millis));
//     };
// }

// /// A clutter helper to flush the stdout on call
// #[macro_export]
// macro_rules! flush {
//     () => {
//         std::io::stdout().flush().expect("Failed to flush stdout");
//     };
// }

// /// A clutter helper to clear the terminal screen
// #[macro_export]
// macro_rules! clear {
//     () => {
//         print!("{}[2J", 27 as char);
//     };
// }

// /// A macro to print a standardised warning. 
// #[macro_export]
// macro_rules! warning {
//     ($msg:literal) => {
//         newln!();
//         println!("{}{}", "WARNING: ".bold().red(), $msg.red());
//         newln!();
//         sleep!(4000);
//     };
// }

// /// Used to signify to the `mint_cli` to continue the game loop.
// pub const GAME_ONGOING: i32 = 31;

// /// Used to signify to the `mint_cli`to restart the game loop.
// pub const GAME_RESTART: i32 = 32;

// /// Used to signify to the `mint_cli` to break the game loop.
// pub const GAME_OVER: i32 = 33;

// const DEFAULT_LIST: ListType = ListType::Gpt;

// // These are all added as const strings as to be embedded in the binary.
// // Any word lists to be added need to be embedded in the binary to be read
// // by the read_word_list function.
// pub const WORDS_4E: &str = include_str!("../word_lists/4E.txt");
// pub const WORDS_4M: &str = include_str!("../word_lists/4M.txt");
// pub const WORDS_4H: &str = include_str!("../word_lists/4H.txt");
// pub const WORDS_5E: &str = include_str!("../word_lists/5E.txt");
// pub const WORDS_5M: &str = include_str!("../word_lists/5M.txt");
// pub const WORDS_5H: &str = include_str!("../word_lists/5H.txt");
// pub const WORDS_6E: &str = include_str!("../word_lists/6E.txt");
// pub const WORDS_6M: &str = include_str!("../word_lists/6M.txt");
// pub const WORDS_6H: &str = include_str!("../word_lists/6H.txt");
// pub const WORDS_7E: &str = include_str!("../word_lists/7E.txt");
// pub const WORDS_7M: &str = include_str!("../word_lists/7M.txt");
// pub const WORDS_7H: &str = include_str!("../word_lists/7H.txt");
// pub const WORDS_8E: &str = include_str!("../word_lists/8E.txt");
// pub const WORDS_8M: &str = include_str!("../word_lists/8M.txt");
// pub const WORDS_8H: &str = include_str!("../word_lists/8H.txt");

// /// This is the `words_alpha.txt` list from the [dwyl/english_words](https://github.com/dwyl/english-words/blob/master/words_alpha.txt)
// /// Github repository. It contains all alpha words. Used to check if a guessed
// /// word actually exists... Considering the size of the list, most guesses probably do...
// pub const WORDS_MASTER: &str = include_str!("../word_lists/Master.txt");

// /// Default methods for a terminal-based game.
// pub trait Game {
//     /// Used to transform the arguments, if any, into the game object.
//     fn new(args: Args) -> Self;

//     /// Starts the game. This is called **once** at the beginning by the
//     /// `mint_cli`.
//     fn start(&mut self);

//     /// What should be done in a loop. This is called in a loop by the
//     /// `mint_cli` until an error occurs, or a code other than `GAME_ONGOING`
//     /// is returned.
//     fn do_loop(&mut self) -> Result<i32, Box<dyn Error>>;

//     /// Finishes the game. Anything to be cleaned up/done **once** at the end
//     /// of the game loop should be done here.
//     fn finish(self);
// }

// #[allow(dead_code)]
// /// Enums for the different word-lists available.
// enum ListType {
//     /// This is all words, only filtered by letter and repeats. Some words **don't exist**.
//     Gpt,

//     /// This contains only words defined by the Webster dictionary. **All words exist, but not all are included.**
//     Webster
// }

// #[derive(ValueEnum, Clone, Debug)]
// /// These are the games available. Game names are parsed through `clap`'s `ValueParser`.
// pub enum Mints {
//     Wordle,
//     Hangman,
//     Anagrams,
//     Minesweeper
// }

// #[derive(ValueEnum, Clone, Debug)]
// /// Three default difficulties for games to make use of.
// pub enum Difficulty {
//     Easy,
//     Medium,
//     Hard,
// }

// #[derive(Parser, Debug, Clone)]
// pub struct Args {
//     #[arg(help = "The game to play.")]
//     pub game: Mints,

//     #[arg(short = 'g', long = "guesses")]
//     pub guesses: Option<i32>,

//     #[arg(short = 'l', long = "letters")]
//     pub letters: Option<i32>,

//     #[arg(short = 't', long = "timer", value_parser = value_parser!(i32).range(1..=300))]
//     pub timer: Option<i32>,

//     #[arg(short = 'd', long = "difficulty")]
//     #[clap(value_enum)]
//     pub difficulty: Option<Difficulty>,
// }

// pub fn choose_random_word(words: &[String]) -> String {
//     let mut rng = rand::thread_rng();
//     words
//         .choose(&mut rng)
//         .cloned()
//         .expect("Failed to pick random word")
// }

// pub fn load_word_list(letters: i32, diff: &Difficulty) -> Vec<String> {
//     let txt = match letters {
//         4 => match diff {
//             Difficulty::Easy => WORDS_4E,
//             Difficulty::Medium => WORDS_4M,
//             Difficulty::Hard => WORDS_4H,
//         },
//         5 => match diff {
//             Difficulty::Easy => WORDS_5E,
//             Difficulty::Medium => WORDS_5M,
//             Difficulty::Hard => WORDS_5H,
//         },
//         6 => match diff {
//             Difficulty::Easy => WORDS_6E,
//             Difficulty::Medium => WORDS_6M,
//             Difficulty::Hard => WORDS_6H,
//         },
//         7 => match diff {
//             Difficulty::Easy => WORDS_7E,
//             Difficulty::Medium => WORDS_7M,
//             Difficulty::Hard => WORDS_7H,
//         },
//         8 => match diff {
//             Difficulty::Easy => WORDS_8E,
//             Difficulty::Medium => WORDS_8M,
//             Difficulty::Hard => WORDS_8H,
//         },
//         _ => unreachable!(),
//     };

//     let words = sanitise_gpt_list(txt, letters);
//     words
// }

// /// As the word lists are generated by GPT-3.5, this function takes the list generated
// /// online using OpenAI's website.
// ///
// /// ## Prompt
// ///
// /// > "ok im doing a word list for wordle, lets start with <4/5/6/7/8> letters, <easy/medium/hard>/
// ///    <common/uncommon/rare> words, 300 words in copyable code block and try your absolute
// ///    hardest to NOT repeat words"
// ///
// /// Once copied into a txt file, this function captures repeats and makes sure each word
// /// is the specified letter amount. It also turns everything to uppercase for use in the game.
// ///
// /// You have two options for santisation here, you can accept all words that meet the above
// /// criteria, or you can accept only the words which exist in the webster dictionary (70-80%).
// /// The dictionary lookup is done by the `webster` crate. Change the `DEFAULT_LIST` parameter
// /// to change which words to accept. Webster doesn't have *all* words, but the GPT list won't
// /// guarantee actual words. Choice is yours. GPT is set by default as it tends to be alright.
// ///
// /// ## Word Counts
// ///
// /// | Difficulty | GPT Count | Webster Count |
// /// |------------|-----------|---------------|
// /// | 4E         | 234       | 198           |
// /// | 4M         | 328       | 263           |
// /// | 4H         | 294       | 142           |
// /// | 5E         | 286       | 229           |
// /// | 5M         | 331       | 264           |
// /// | 5H         | 489       | 259           |
// /// | 6E         | 232       | 172           |
// /// | 6M         | 211       | 156           |
// /// | 6H         | 231       | 148           |
// /// | 7E         | 329       | 153           |
// /// | 7M         | 247       | 131           |
// /// | 7H         | 322       | 137           |
// /// | 8E         | 112       | 74            |
// /// | 8M         | 184       | 80            |
// /// | 8H         | 122       | 76            |
// ///
// /// ## Overlap Counts
// /// | Letters | All Difficulties | Easy & Medium | Easy & Difficult | Medium & Difficult |
// /// |---------|------------------|---------------|------------------|--------------------|
// /// | 4       | 1                | 23            | 8                | 26                 |
// /// | 5       | 29               | 147           | 30               | 158                |
// /// | 6       | 10               | 51            | 16               | 36                 |
// /// | 7       | 12               | 64            | 26               | 44                 |
// /// | 8       | 0                | 19            | 0                | 1                  |
// ///
// /// Because these are all generated with ChatGPT, it doesn't always isolate
// /// words to the difficulty categories.
// ///
// fn sanitise_gpt_list(list: &str, letters: i32) -> Vec<String> {
//     let mut repeat_map = HashMap::new();

//     for line in list.lines() {
//         for word in line.split_ascii_whitespace().map(|s| s.trim()) {
//             if word.len() == letters as usize && word.chars().all(|c| c.is_ascii_alphabetic()) {
//                 *repeat_map.entry(word.to_ascii_uppercase()).or_insert(0) += 1;
//             }
//         }
//     }

//     let mut webster_list = Vec::new();

//     for word in repeat_map.keys() {
//         if webster::dictionary(word).is_some() {
//             webster_list.push(word.clone());
//         }
//     }

//     // dbg!(&repeat_map);
//     // dbg!(&webster_list);
//     // println!("Unique words: {}", repeat_map.keys().len());
//     // println!("Defined words: {}", webster_list.len());
//     // panic!();

//     let gpt_list: Vec<String> = repeat_map.keys().map(|s| s.to_owned()).collect();

//     match DEFAULT_LIST {
//         ListType::Gpt => gpt_list,
//         ListType::Webster => webster_list,
//     }
// }

// pub fn word_exists(letters: i32, word: &String) -> bool {
//     sanitise_gpt_list(WORDS_MASTER, letters).contains(word)
//         || load_word_list(letters, &Difficulty::Easy).contains(word)
//         || load_word_list(letters, &Difficulty::Medium).contains(word)
//         || load_word_list(letters, &Difficulty::Hard).contains(word)
// }

// pub fn define(word: &String) -> String {
//     webster::dictionary(word)
//         .unwrap_or("No definition found!")
//         .to_string()
// }

// pub fn hint(word: &String) -> String {
//     let word = word.to_ascii_lowercase();
//     let definition = define(&word);
//     let filler = "_".repeat(word.len());
//     definition.replace(&word, &format!("[{filler}]"))
// }

// /// Prints a given header in block form, in the fashion of a pixel-game loading screen.
// /// Specify the desired loading time by passing in `ms` in milliseconds. This function will
// /// **not** clear the terminal after it finishes, leaving it to the game to handle when the loading
// /// screen should clear.
// pub fn titled_loading_screen(header: &str, color: &str, ms: usize) {
//     let middle = if let Some((_, terminal_size::Height(h))) = terminal_size::terminal_size() {
//         (h as usize / 2) - 3
//     } else {
//         0 // Default height in case terminal size can't be determined
//     };

//     let hold_time = ms / 4;
//     let print_time = ((ms / 4) * 3) / header.len();
//     let header = header.to_ascii_uppercase();

//     clear!();
//     for (i, _) in header.chars().enumerate() {

//         // if i < 8 {
//             print!(
//                 "{}",
//                 terminal_fonts::to_block_string(&header[0..=i]).color(color)
//             );
//         // } else {
//         //     print!(
//         //         "{}",
//         //         terminal_fonts::to_block_string(&header[8..=i]).color(color)
//         //     );
//         // }
//         newln!(middle);
//         flush!();

//         if i == header.len() - 1 {
//             sleep!(hold_time as u64);
//         } else {
//             sleep!(print_time as u64);
//             clear!();
//         }
//         newln!();
//     }
// }

// // Dev function used to check the overlap of word-lists.
// // pub fn check_overlap(a: &Vec<String>, b: &Vec<String>, c: &Vec<String>) {
// //     let set_a: HashSet<_> = a.into_iter().collect();
// //     let set_b: HashSet<_> = b.into_iter().collect();
// //     let set_c: HashSet<_> = c.into_iter().collect();

// //     // Intersection of all three sets
// //     let abc_intersection = set_a
// //         .intersection(&set_b)
// //         .map(|s| s.deref())
// //         .collect::<HashSet<&String>>() // Collects references
// //         .intersection(&set_c)
// //         .count();
// //     println!("Words in all three lists: {}", abc_intersection);

// //     // Intersection of A and B
// //     let ab_intersection = set_a.intersection(&set_b).count();
// //     println!("Words in both A and B: {}", ab_intersection);

// //     // Intersection of A and C
// //     let ac_intersection = set_a.intersection(&set_c).count();
// //     println!("Words in both A and C: {}", ac_intersection);

// //     // Intersection of B and C
// //     let bc_intersection = set_b.intersection(&set_c).count();
// //     println!("Words in both B and C: {}", bc_intersection);
// // }

// // for letters in 4..=8 {
// //     println!("LETTERS: {letters}");
// //     let a = load_word_list(letters, &Difficulty::Easy);
// //     let b = load_word_list(letters, &Difficulty::Medium);
// //     let c = load_word_list(letters, &Difficulty::Hard);

// //     check_overlap(&a, &b, &c);
// // }
// // panic!();
