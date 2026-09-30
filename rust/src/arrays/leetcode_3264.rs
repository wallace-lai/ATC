struct Solution;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _ in 0..k {
            let min = nums.iter_mut().min();
            if let Some(i) = min {
                *i = *i * multiplier;
            }
        }
        nums
    }
}