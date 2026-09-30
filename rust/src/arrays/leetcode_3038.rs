struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let sum = nums[0] + nums[1];
        let n = nums.len();

        let mut i = 2;
        while i + 1 < n {
            if nums[i] + nums[i + 1] == sum {
                ans += 1;
            } else {
                break;
            }
            i += 2;
        }

        ans
    }
}