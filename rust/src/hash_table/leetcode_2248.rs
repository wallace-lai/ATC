struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let max_len = nums.iter().map(|v| v.len()).max().unwrap();
        let mut count = HashMap::with_capacity(max_len);
        for num in nums.iter().flatten() {
            *count.entry(*num).or_insert(0) += 1;
        }

        let mut ans = vec![];
        for (&key, &val) in count.iter() {
            if val as usize == nums.len() {
                ans.push(key);
            }
        }

        ans.sort_unstable();
        ans
    }
}