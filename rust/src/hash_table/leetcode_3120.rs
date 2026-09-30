struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut set = HashSet::with_capacity(word.len());
        for c in word.chars() {
            set.insert(c);
        }

        let mut ans = 0;
        for i in 0..26_u8 {
            let upper = (i + b'A') as char;
            let lower = (i + b'A' + 32) as char;
            if set.contains(&upper) && set.contains(&lower) {
                ans += 1;
            }
        }
        ans
    }
}