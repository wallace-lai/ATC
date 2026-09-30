struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let len = nums.len();
        let min = nums[0];
        let max = nums[len - 1];

        let set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = Vec::with_capacity(len);
        for i in min..max {
            if !set.contains(&i) {
                ans.push(i);
            }
        }

        ans
    }
}