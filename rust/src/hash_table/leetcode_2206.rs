struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut count = HashMap::with_capacity(nums.len());
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        for val in count.values() {
            if *val & 1 != 0 {
                return false;
            }
        }

        true
    }
}