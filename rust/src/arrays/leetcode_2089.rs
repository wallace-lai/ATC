struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        let mut v = Vec::new();

        for i in 0..nums.len() {
            if nums[i] == target {
                v.push(i as i32);
            }
        }

        v
    }
}