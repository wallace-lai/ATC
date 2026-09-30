struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let b: HashSet<String> = banned.into_iter()
            .map(|s| s.to_lowercase())
            .collect();
    
        let mut m = HashMap::new();
        for word in paragraph.split(|c: char| !c.is_ascii_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase()) {
            *m.entry(word).or_insert(0_i32) += 1;
        }

        let result = m.into_iter()
            .filter(|(word, _)| !b.contains(word))
            .max_by_key(|(_, count)| *count)
            .map(|(word, _)| word)
            .unwrap_or_else(|| "".to_string());

        result
    }
}