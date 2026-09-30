struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn expect_number(scores: Vec<i32>) -> i32 {
        let s: HashSet<i32> = scores.into_iter().collect();
        s.len() as i32
    }
}