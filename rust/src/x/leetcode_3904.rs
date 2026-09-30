struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 1 { return 0; }

        let mut pre = i32::MIN;
        let mut suf = vec![0; n];

        suf[n - 1] = nums[n - 1];
        for i in (0..=(n - 2)).rev() {
            suf[i] = nums[i].min(suf[i + 1]);
        }

        for i in 0..n {
            pre = pre.max(nums[i]);
            if pre - suf[i] <= k {
                return i as i32
            }
        }

        -1
    }
}