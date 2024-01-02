use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;
use super::*;

#[derive(Clone, Debug, Default)]
pub struct Anagram {
    pub scramble: String,

    // key is number of letters and value is list of words that can be made from the scramble
    pub words: HashMap<usize, Vec<String>>
}

impl Anagram {
    pub fn new(list: &str, size: usize, params: &AnagramParams) -> Self {
        let mut scrambles = HashSet::new();
        for word in list.split_ascii_whitespace() {
            if word.len() == size {
                let mut scramble: Vec<char> = word.chars().collect();
                scramble.sort();
                let scramble = scramble
                    .iter()
                    .fold("".to_string(), |mut s , c| { s.push(c.to_owned()); s });
                scrambles.insert(scramble);
            }
        }

        let scrambles: Vec<String> = scrambles.into_iter().collect();
        let mut scramble = scrambles
            .choose(&mut rand::thread_rng())
            .expect("Failed to pick Scramble")
            .to_owned();
        let mut words = Self::words_from_scramble(&scramble, list);

        while !Self::valid_anagrams(&words, params) {
            // println!("Invalid scramble");
            scramble = scrambles
                .choose(&mut rand::thread_rng())
                .expect("Failed to pick Scramble")
                .to_owned();
            words = Self::words_from_scramble(&scramble, list);
        }

        // This was used to count all possible anagrams from the parameters.
        // let mut anagrams = Vec::new();
        // for (i, scramble) in scrambles.iter().enumerate() {
        //     let words = Self::words_from_scramble(&scramble, list, params);
        //     println!("{} of {} || {} valid", i, scrambles.len(), anagrams.len());
        //     if Self::valid_anagrams(&words, params) {
        //         let anagram = Anagram {
        //             scramble: scramble.chars().collect::<Vec<_>>().choose_multiple(&mut rand::thread_rng(), scramble.len()).collect(),
        //             words
        //         };
        //         dbg!(&anagram);
        //         anagrams.push(anagram);
        //     }
        // }
        // dbg!(&anagrams.len());
        // todo!();

        Anagram {
            // Rescramble of the chars so they aren't in alphabetical (which was used to remove duplicates)
            scramble: scramble.chars().collect::<Vec<_>>().choose_multiple(&mut rand::thread_rng(), scramble.len()).collect(),
            words,
        }
    }

    fn words_from_scramble(scramble: &str, input: &str) -> HashMap<usize, Vec<String>> {
        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    
        // Count occurrences of each character in the scramble
        let mut char_counts: HashMap<char, usize> = HashMap::new();
        for c in scramble.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
    
        // Filter words that can be built from the scramble with correct letter counts
        let mut result: HashMap<usize, Vec<String>> = HashMap::new();
        for word in words {
            let mut remaining_chars: HashMap<char, usize> = char_counts.clone();
            let mut is_subset = true;
    
            for c in word.chars() {
                if let Some(count) = remaining_chars.get_mut(&c) {
                    if *count > 0 {
                        *count -= 1;
                    } else {
                        is_subset = false;
                        break;
                    }
                } else {
                    is_subset = false;
                    break;
                }
            }
    
            if is_subset {
                let entry = result.entry(word.len()).or_insert_with(Vec::new);
                entry.push(word);
            }
        }
    
        result
    }

    fn valid_anagrams(words: &HashMap<usize, Vec<String>>, params: &AnagramParams) -> bool {
        for i in params.letter_range.clone() {
            if !words.contains_key(&i) {
                return false;
            }
        }

        for (letters, anagrams) in words.iter() {
            if params.letter_range.contains(letters) {
                if anagrams.len() < params.entry_min(*letters) {
                    return false;
                }
            }
        }
        true
    }

    pub fn valid_word(&self, word: &String) -> bool {
        match self.words.get(&word.len()) {
             Some(e) => e.contains(word),
             None => false,
         }
     }

    // THE BELOW ARE OBSOLETE METHODS OF GENERATING ANAGRAMS ARCHAICLY. 
    // pub fn generate_anagrams(input: &str, scramble_limit: usize) -> Vec<Anagram> {
    //     let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    
    //     // Create a HashSet of unique characters from the input words
    //     let unique_chars: HashSet<char> = words.iter().flat_map(|word| word.chars()).collect();
    
    //     // Generate anagrams with different keys
    //     let anagrams: Vec<Anagram> = (4..=8)
    //         .filter_map(|letters| {
    //             // Find all words with the specified number of letters
    //             let valid_words: Vec<String> = words
    //                 .iter()
    //                 .filter(|word| word.len() == letters)
    //                 .cloned()
    //                 .collect();
    
    //             if valid_words.is_empty() {
    //                 None
    //             } else {
    //                 // Generate a scramble by randomly selecting characters from unique_chars
    //                 let scramble: String = unique_chars
    //                     .iter()
    //                     .cycle()
    //                     .take(scramble_limit)
    //                     .cloned()
    //                     .collect();
    
    //                 // Create Anagram type
    //                 let mut words_map = HashMap::new();
    //                 words_map.insert(letters as i32, valid_words);
    
    //                 Some(Anagram { scramble, words: words_map })
    //             }
    //         })
    //         .collect();
    
    //     anagrams
    // }
    
    // pub fn get_letter_words(letters: i32) -> Vec<String> {
    //     let mut list = Vec::new();
    //     for word in WORDS_MASTER.split_ascii_whitespace() {
    //         if word.len() == letters as usize {
    //             list.push(word.to_string());
    //         }
    //     }
    //     list
    // }

    // fn calculate_key(word: &str) -> String {
    //     let mut chars: Vec<char> = word.chars().collect();
    //     chars.sort();
    //     chars.into_iter().collect()
    // }
    
    // pub fn find_anagrams(input: &str) -> Vec<Vec<Anagram>> {
    //     let words: Vec<&str> = input.split_whitespace().collect();
    
    //     // Group words by length
    //     let mut length_groups: HashMap<usize, Vec<&str>> = HashMap::new();
    //     for word in words.iter().cloned() {
    //         length_groups.entry(word.len()).or_insert_with(Vec::new).push(word);
    //     }
    
    //     // Calculate anagrams for lengths 4 to 8
    //     let mut anagrams: Vec<Vec<Anagram>> = Vec::new();
    //     for length in ANAGRAMS_LEVELS_RANGE {
    //         if let Some(group) = length_groups.get(&length) {
    //             let mut anagram_entries: HashMap<String, Vec<String>> = HashMap::new();
    //             for &word in group {
    //                 let key = Self::calculate_key(word);
    //                 anagram_entries.entry(key).or_insert_with(Vec::new).push(word.to_string());
    //             }
    
    //             let mut anagram_group: Vec<Anagram> = anagram_entries
    //                 .into_iter()
    //                 .map(|(key, words)| Anagram {
    //                     letters: length as i32,
    //                     key: key.clone(),
    //                     words: words.clone(),
    //                     entries: words.len() as i32,
    //                 })
    //                 .filter(|anagram| anagram.entries >= ANAGRAM_MIN_ENTRIES as i32) // Filter out anagrams with insufficient entries
    //                 .collect();
    
    //             anagram_group.sort_by(|a, b| b.entries.cmp(&a.entries));
    //             anagrams.push(anagram_group);
    //         } else {
    //             anagrams.push(Vec::new());
    //         }
    //     }
    
    //     anagrams
    // }
}
