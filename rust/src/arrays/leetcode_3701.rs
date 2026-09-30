struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .map(|(i, num)| { if i & 1 == 1 { -num } else { num } })
            .sum()
    }
}