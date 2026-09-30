struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        let mut i = nums.len() as i32 - 1;

        while i >= 0 {
            if set.contains(&nums[i as usize]) {
                break;
            }

            set.insert(nums[i as usize]);
            i -= 1;
        }

        (i + 2) / 3
    }
}