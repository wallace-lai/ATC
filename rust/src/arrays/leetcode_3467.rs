struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] & 1 == 0 {
                nums[idx] = 0;
                idx += 1;
            }
        }

        if let Some(slice) = nums.get_mut(idx..) {
            slice.fill(1);
        }

        nums
    }
}