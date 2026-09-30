struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = HashMap::new();
        for i in 0..(nums.len() - 1) {
            if nums[i] == key {
                *count.entry(nums[i + 1]).or_insert(0) += 1;
            }
        }

        let mut max_cnt = 0;
        let mut ans = 0;
        for (key, val) in count {
            if val > max_cnt {
                max_cnt = val;
                ans = key;
            }
        }

        ans
    }
}