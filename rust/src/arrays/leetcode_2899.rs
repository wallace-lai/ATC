struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = VecDeque::with_capacity(nums.len());
        let mut ans = Vec::with_capacity(nums.len());

        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] > 0 {
                seen.push_front(nums[i]);
                k = 0;
            } else {
                k += 1;
                if k as usize <= seen.len() {
                    ans.push(seen[k as usize - 1]);
                } else {
                    ans.push(-1);
                }
            }
        }

        ans      
    }
}