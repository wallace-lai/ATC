struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.windows(2).all(|w| w[0] == w[1]) {
            0
        } else {
            1
        }
    }
}