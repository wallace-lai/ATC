struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m = HashMap::new();
        for s in strs.iter() {
            let mut key = [0; 26];
            for &c in s.as_bytes() {
                let idx = (c - b'a') as usize;
                key[idx] += 1;
            }

            m.entry(key).or_insert(Vec::new()).push(s.clone());
        }

        let mut ans = Vec::new();
        for (_, v) in m.into_iter() {
            ans.push(v);
        }

        ans
    }
}