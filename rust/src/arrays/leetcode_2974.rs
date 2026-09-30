struct Solution;

impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = Vec::with_capacity(n);

        for i in (0..n).step_by(2) {
            ans.push(nums[i + 1]);
            ans.push(nums[i]);
        }

        ans
    }
}