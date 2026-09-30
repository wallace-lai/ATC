struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
            "....", "..", ".---", "-.-", ".-..", "--", "-.",
            "---", ".--.", "--.-", ".-.", "...", "-",
            "..-", "...-", ".--", "-..-", "-.--", "--.."
        ];
        let mut s = HashSet::with_capacity(words.len());
        
        for word in words {
            let mut code = String::with_capacity(word.len() * 4);
            for &c in word.as_bytes() {
                let idx = (c - b'a') as usize;
                code.push_str(morse[idx]);
            }

            s.insert(code);
        }
        
        s.len() as i32
    }
}