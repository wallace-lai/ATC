struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn maximize_expression_of_three(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort_unstable();
        nums[len - 1] + nums[len - 2] - nums[0]
    }
}