struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut set:HashSet<i32> = (1..=k).into_iter().collect();
        let mut ans = 0;
        while nums.len() > 0 && !set.is_empty() {
            ans += 1;

            if let Some(last) = nums.pop() {
                set.remove(&last);
            }
        }

        ans
    }
}