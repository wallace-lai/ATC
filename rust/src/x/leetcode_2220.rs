struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let a = start as u32;
        let b = goal as u32;
        (a ^ b).count_ones() as i32
    }
}