struct Solution;

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];
        for i in 1..nums.len() {
            if nums[i] < min {
                min = nums[i];
            }
            if nums[i] > max {
                max = nums[i];
            }
        }

        for i in 0..nums.len() {
            if nums[i] != min && nums[i] != max {
                return nums[i];
            }
        }

        -1
    }
}