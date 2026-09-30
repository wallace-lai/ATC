struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut sum = vec![0; nums.len()];
        sum[0] = nums[0];
        for i in 1..len {
            sum[i] = nums[i] + sum[i - 1];
        }

        let mut ans = 0;
        for i in 0..(len - 1) {
            let sub = 2 * sum[i] - sum[len - 1];
            if sub & 1 == 0 { ans += 1; }
        }

        ans
    }
}