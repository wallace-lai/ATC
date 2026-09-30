struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let n = nums.len() as i32;
        let mut abs = 0;

        loop {
            if start - abs >= 0 && nums[(start - abs) as usize] == target {
                return abs;
            }
            if start + abs < n && nums[(start + abs) as usize] == target {
                return abs;
            }
            abs += 1;
        }
    }
}