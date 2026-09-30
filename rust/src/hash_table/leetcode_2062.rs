struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    // O(n) - 0ms
    // 为什么下面这个滑动窗口的解法是对的？
    pub fn count_vowel_substrings(word: String) -> i32 {
        let vowels: HashSet<u8> = [b'a', b'e', b'i', b'o', b'u'].into_iter().collect();
        let mut window = HashMap::with_capacity(8);
        let mut ans = 0;
        let mut start = 0;
        let mut left = 0;

        for (i, val) in word.as_bytes().iter().enumerate() {
            if !vowels.contains(val) {
                window.clear();
                start = i + 1;
                left = i + 1;
                continue;
            }

            *window.entry(*val).or_insert(0) += 1;
            while window.len() == 5 {
                let out = word.as_bytes()[left];
                if let Some(val) = window.get_mut(&out) {
                    *val -= 1;
                    if *val == 0 {
                        window.remove(&out);
                    }
                }
                left += 1;
            }

            ans += left - start;
        }

        ans as i32
    }
}