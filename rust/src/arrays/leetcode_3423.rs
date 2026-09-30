struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut ans = nums[len - 1].abs_diff(nums[0]);
        for i in 0..(len - 1) {
            ans = ans.max(
                nums[i].abs_diff(nums[i + 1])
            );
        }

        ans as i32
    }
}