struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![0; nums.len()];
        for i in 0..nums.len() {
            v[i] = nums[nums[i] as usize];
        }
        v
    }
}