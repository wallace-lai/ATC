struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::with_capacity(nums.len());
        for num in nums {
            if set.contains(&num) {
                return num;
            }
            set.insert(num);
        }

        return -1;
    }
}