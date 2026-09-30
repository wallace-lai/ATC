struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn has_broken_letters(w: &str, s: &HashSet<u8>) -> bool {
        for c in w.as_bytes() {
            if s.contains(&c) {
                return true;
            }
        }

        false
    }

    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let s: HashSet<u8> = broken_letters.as_bytes()
            .iter()
            .copied()
            .collect();

        let mut ans = 0;
        for word in text
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty()) {
            if !Self::has_broken_letters(word, &s) {
                ans += 1;
            }
        }

        ans
    }
}