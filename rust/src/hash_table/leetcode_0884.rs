struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut m1 = HashMap::new();
        for s in s1.split_whitespace()
            .filter(|&s| !s.is_empty()) {
            *m1.entry(s).or_insert(0) += 1;
        }

        let mut m2 = HashMap::new();
        for s in s2.split_whitespace()
            .filter(|&s| !s.is_empty()) {
            *m2.entry(s).or_insert(0) += 1;
        }

        // println!("v is {:?}", v);
        let mut ans = vec![];
        for (k, v) in m1.iter() {
            if *v == 1 && !m2.contains_key(k) {
                ans.push(k.to_string());
            }
        }
        for (k, v) in m2.iter() {
            if *v == 1 && !m1.contains_key(k) {
                ans.push(k.to_string());
            }
        }

        ans
    }
}