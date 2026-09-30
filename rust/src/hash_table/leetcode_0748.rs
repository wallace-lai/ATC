
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_complete(map: &HashMap<char, i32>, count: &HashMap<char, i32>) -> bool {
        for (k, v1) in map {
            let value = match count.get(k) {
                Some(&val) => val,
                None => 0,
            };
            if value < *v1 { return false; }
        }

        true
    }

    // 0ms，击败100%
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut map = HashMap::new();
        for &c in license_plate.as_bytes() {
            if !c.is_ascii_alphabetic() { continue; }
            let mut char = c as char;
            if char.is_ascii_uppercase() {
                char = char.to_ascii_lowercase();
            }
            *map.entry(char).or_insert(0) += 1;
        }   

        let mut min_len = usize::MAX;
        let mut ans = words[0].as_str();
        let mut count = HashMap::new();
        for i in 0..words.len() {
            count.clear();
            for c in words[i].chars() {
                *count.entry(c).or_insert(0) += 1;
            }
            if Self::is_complete(&map, &count) && words[i].len() < min_len {
                min_len = words[i].len();
                ans = words[i].as_str();
            }
        }

        ans.to_string()
    }
}