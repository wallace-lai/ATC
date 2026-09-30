struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for w in nums.windows(2) {
            let sum: i32 = w.iter().sum();
            if set.contains(&sum) {
                return true;
            }
            set.insert(sum);
        }

        false
    }
}