struct Solution;

use std::cmp::min;
use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let half_n = candy_type.len() / 2;
        let s = candy_type.into_iter().collect::<HashSet<i32>>();

        min(half_n, s.len()) as i32
    }
}