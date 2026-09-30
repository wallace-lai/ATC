struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len() * 2);
        ans.extend(&nums);
        ans.extend(nums.iter().rev());
        ans
    }
}