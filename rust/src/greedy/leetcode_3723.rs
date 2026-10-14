struct Solution;

impl Solution {
    pub fn max_sum_of_squares(mut num: i32, mut sum: i32) -> String {
        if sum > num * 9 { return "".to_string(); }
        let mut v = vec![b'0'; num as usize];

        let mut i = 0;
        while sum >= 9 {
            v[i] += 9;
            sum -= 9;
            i += 1;
        }
        if sum > 0 { v[i] += sum as u8; }

        unsafe { String::from_utf8_unchecked(v) }
    }
}