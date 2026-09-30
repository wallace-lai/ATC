struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|num| *num == 0) { return 0; }
        let xor = nums.iter().fold(0, |n, i| n ^ *i);
        let ans = if xor != 0 { nums.len() as i32 } else { nums.len() as i32 - 1 };
        ans
    }
}