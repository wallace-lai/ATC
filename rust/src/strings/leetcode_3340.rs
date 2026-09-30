struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut sum1 = 0;
        let mut sum2 = 0;
        
        for i in (0..num.len()).step_by(2) {
            sum1 += (num.as_bytes()[i] - b'0') as i32;
        }

        for i in (1..num.len()).step_by(2) {
            sum2 += (num.as_bytes()[i] - b'0') as i32;
        }

        sum1 == sum2
    }
}