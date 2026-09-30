struct Solution;

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        let mut ans = -1;

        for i in 0..nums.len() {
            if i as i32 % 10 == nums[i] {
                ans = i as i32;
                break;
            }
        }

        ans
    }
}