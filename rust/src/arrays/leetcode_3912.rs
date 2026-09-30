struct Solution;

use std::cmp::max;

impl Solution {
    // 0ms，击败100%
    pub fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len < 3 { return nums; }

        let mut pre = vec![0; len];
        let mut suf = vec![0; len];
        for i in 1..len {
            pre[i] = max(pre[i - 1], nums[i - 1]);
        }
        for i in (0..=(len - 2)).rev() {
            suf[i] = max(suf[i + 1], nums[i + 1]);
        }


        let mut ans = Vec::with_capacity(len);
        for i in 0..len {
            if nums[i] > pre[i] || nums[i] > suf[i] {
                ans.push(nums[i]);
            }
        }

        ans
    }
}