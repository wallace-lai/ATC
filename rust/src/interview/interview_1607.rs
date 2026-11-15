struct Solution;

impl Solution {
    pub fn maximum(a: i32, b: i32) -> i32 {
        let d = a as i64 - b as i64;
        let k = (d >> 63) as i32;
        (k + 1) * a - k * b
    }
}