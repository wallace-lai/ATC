struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::new();
        for i in 0..nums.len() {
            let idx = index[i] as usize;
            v.insert(idx, nums[i]);
        }

        v
    }
}