struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(nums.len() * 2);

        for i in (0..nums.len()).step_by(2) {
            let freq = nums[i] as usize;
            v.resize(v.len() + freq, nums[i + 1]);
        }

        v
    }
}