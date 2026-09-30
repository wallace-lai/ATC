struct Solution;

impl Solution {
    pub fn is_middle_element_unique(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let key = nums[n / 2];
        for i in 0..n {
            if nums[i] == key && i != n / 2 {
                return false;
            }
        }
        true
    }
}