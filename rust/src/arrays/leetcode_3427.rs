struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0; nums.len()];
        sum[0] = nums[0];
        for i in 1..nums.len() {
            sum[i] = nums[i] + sum[i - 1];
        }

        let mut ans = 0;
        for i in 0..nums.len() {
            let start = 0.max(i as i32 - nums[i]);
            if start - 1 >= 0 {
                ans += sum[i] - sum[start as usize - 1];
            } else {
                ans += sum[i];
            }
        }

        ans
    }
}