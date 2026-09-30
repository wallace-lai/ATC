struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        nums.into_iter()
        .filter(|&x| x & 1 == 0)
        .fold(0, |acc, x| acc | x)
    }
}