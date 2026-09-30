struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..digits.len() {
            for j in 0..digits.len() {
                if i == j { continue; }
                for k in 0..digits.len() {
                    if i == k || j == k { continue; }
                    if digits[i] == 0 || digits[k] & 1 == 1 {
                        continue;
                    }
                    set.insert(digits[i] * 100 + digits[j] * 10 + digits[k]);
                }
            }
        }

        set.len() as i32
    }
}