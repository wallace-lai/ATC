struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().copied().collect();
        let sum: i32 = nums.iter().sum();
        let avg = sum / nums.len() as i32;

        let mut ans = avg + 1;
        while set.contains(&ans) || ans <= 0 {
            ans += 1;
        }

        ans
    }
}