struct Solution;

use std::cmp::max;
use std::cmp::min;

impl Solution {
    // 0ms，击败100%
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        assert!(len >= 1);
        let mut pre = vec![nums[0]; len];
        let mut suf = vec![nums[len - 1]; len];

        for i in 1..len {
            pre[i] = max(pre[i - 1], nums[i]);
        }
        for i in (0..(len - 1)).rev() {
            suf[i] = min(suf[i + 1], nums[i]);
        }

        let mut ans = -1;
        for i in 0..len {
            if pre[i] - suf[i] <= k {
                ans = i as i32;
                break;
            }
        }

        ans
    }
}