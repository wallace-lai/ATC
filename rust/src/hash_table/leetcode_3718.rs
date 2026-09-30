struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();

        let mut x = k;
        while set.contains(&x) {
            x += k;
        }

        x
    }
}