struct Solution;

use std::collections::HashMap;

impl Solution {
    // 0ms，击败100%
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::with_capacity(order.len());
        for (i, key) in order.chars().enumerate() {
            let val = (i as u8 + b'a') as char;
            map.entry(key).or_insert(val);
        }

        let v: Vec<String> = words.into_iter()
            .map(|s| {
                let str: String = s.chars()
                    .map(|c| *map.get(&c).unwrap_or(&c))
                    .collect();
                str
            }).collect();
        
        // println!("v is {:?}", v);

        v.is_sorted()
    }
}