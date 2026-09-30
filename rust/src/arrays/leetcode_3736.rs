struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().unwrap();
        let mut ans = 0;
        for num in nums.iter() {
            ans += *max - *num;
        }

        ans
    }
}