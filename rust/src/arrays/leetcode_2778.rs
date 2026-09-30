struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        for i in 0..nums.len() {
            if n % (i + 1) == 0 {
                ans += nums[i] * nums[i];
            }
        }
        ans
    }
}