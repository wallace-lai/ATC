struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut set = HashSet::new();
        for s in word.split(|c: char| c.is_ascii_alphabetic())
            .filter(|part| !part.is_empty())
            .map(|part| part.trim_start_matches('0')) {
            let num = if s.len() == 0 { "0".to_string() } else { s.to_string() };
            if !set.contains(&num) {
                set.insert(num);
            }
        }
        set.len() as i32
    }
}