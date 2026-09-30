
struct Solution;

use std::cmp::max;
use std::collections::HashMap;

// 法一：哈希表
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        for &i in &nums {
            *map.entry(i).or_insert(0) += 1;
        }

        let mut result = 0;
        for (&key, &value) in &map {
            let len = if let Some(&next_value) = map.get(&(key + 1)) {
                value + next_value
            } else {
                0
            };

            result = max(result, len);
        }

        result
    }
}

