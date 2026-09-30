struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len < 2 { return vec![1]; }
        // pre[i] : [0..i]中不同元素的个数
        // suf[i] : [i+1..n)中不同元素的个数
        let mut pre = vec![0; len];
        let mut suf = vec![0; len];
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());

        for i in 0..len {
            set.insert(nums[i]);
            pre[i] = set.len();
        }
        set.clear();
        for i in (0..=(len - 2)).rev() {
            set.insert(nums[i + 1]);
            suf[i] = set.len();
        }

        let mut ans = vec![0; len];
        for i in 0..len {
            ans[i] = (pre[i] - suf[i]) as i32;
        }

        ans
    }
}