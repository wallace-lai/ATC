struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let prod: i32 = nums.iter()
            .map(|&x| { if x > 0 { 1 } else if x < 0 { -1 } else { 0 } })
            .fold(1, |acc, x| acc * x);
        if prod > 0 {
            1
        } else if prod < 0 {
            -1
        } else {
            0
        }
    }
}