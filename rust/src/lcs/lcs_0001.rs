struct Solution;

impl Solution {
    pub fn least_minutes(n: i32) -> i32 {
        let exp = (n as u32).next_power_of_two();
        (exp as f32).log2() as i32 + 1
    }
}