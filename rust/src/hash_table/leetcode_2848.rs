struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::with_capacity(nums.len());
        for num in nums {
            let start = num[0];
            let end = num[1];
            for i in start..=end {
                set.insert(i);
            }
        }

        set.len() as i32
    }
}