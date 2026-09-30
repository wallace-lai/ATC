struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        for i in 1..nums.len() {
            if nums[i] < min1 {
                min2 = min1;
                min1 = nums[i];
            } else if nums[i] < min2 {
                min2 = nums[i];
            }
        }

        nums[0] + min1 + min2
    }
}