use std::{collections::HashMap, ops::RangeInclusive};

use mints::Difficulty;

#[derive(Clone)]
pub struct AnagramParams {
    pub letter_range: RangeInclusive<usize>,
    pub difficulty: Difficulty,
    entry_amounts: HashMap<usize, (usize, usize, usize)>,
}

impl AnagramParams {
    pub fn new(diff: &Difficulty, guesses: Option<i32>, letters: Option<i32>) -> Self {
        let mut map = HashMap::new();

        // Blitz mode:
        if let (Some(guesses), Some(letters)) = (guesses, letters) {
            let range = letters as usize..=letters as usize;
            map.insert(letters as usize, (guesses as usize, 0, 0));

            return AnagramParams {
                letter_range: range,
                difficulty: Difficulty::Easy,
                entry_amounts: map,
            };
        } 

        let range = match diff {
            Difficulty::Easy => 4..=6,
            Difficulty::Medium => 5..=7,
            Difficulty::Hard => 5..=8,
        };

        for i in range.clone() {
            let insert = match i {
                4 => (3, 0, 0),
                5 => (2, 4, 5),
                6 => (1, 3, 4),
                7 => (0, 2, 3),
                8 => (0, 0, 2),
                _ => panic!("Check that ranges and entries match!"),
            };
            map.insert(i, insert);
        }

        AnagramParams {
            letter_range: range,
            difficulty: diff.clone(),
            entry_amounts: map,
        }
    }

    pub fn entry_min(&self, letters: usize) -> usize {
        let entry_amounts = self.entry_amounts.get(&letters).expect("Ranges unmatched!");
        match self.difficulty {
            Difficulty::Easy => entry_amounts.0,
            Difficulty::Medium => entry_amounts.1,
            Difficulty::Hard => entry_amounts.2,
        }
    }
}
