struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut map = HashMap::with_capacity(len);

        let mut ans = 0;
        for start in 0..len {
            map.clear();
            for end in start..len {
                *map.entry(nums[end]).or_insert(0) += 1;
                ans += map.len() * map.len();
            }
        }

        ans as i32
    }
}