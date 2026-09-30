struct Solution;

impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        let max = nums.iter().max().unwrap();
        let n = *max as usize;
        if nums.len() != (n + 1) { return false; }

        nums.sort_unstable();
        if nums[n] != n as i32 || nums[n - 1] != n as i32 { return false; }

        for i in 0..(n - 1) {
            if nums[i] != (i + 1) as i32 { return false; }
        }

        true
    }
}