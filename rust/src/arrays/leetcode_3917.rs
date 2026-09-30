struct Solution;

impl Solution {
    // O(n)，0ms，击败100%
    pub fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len == 1 { return vec![0]; }

        // dp[i]表示nums右侧范围[i + 1 .. len - 1)内奇数的个数
        let mut dp = vec![0; len];
        let mut ans = vec![0; len];

        for i in (0..=(len - 2)).rev() {
            dp[i] = if nums[i + 1] & 1 == 1 { 1 } else { 0 } + dp[i + 1];

            if nums[i] & 1 == 1 {
                // 奇数，找右侧偶数个数
                let total = (len - i - 1) as i32;
                ans[i] = total - dp[i];
            } else {
                // 偶数，找右侧奇数的个数
                ans[i] = dp[i];
            }
        }

        ans
    }
}