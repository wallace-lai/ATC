struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let k = k as usize;
        let mut ans = 0;
        for i in 0..k {
            ans += nums[nums.len() - 1 - i] - nums[i];
        }

        ans
    }
}