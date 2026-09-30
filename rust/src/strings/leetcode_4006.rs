struct Solution;

impl Solution {
    pub fn count_valid_prefixes(s: String) -> i32 {
        let mut zero: i32 = 0;
        let mut ones: i32 = 0;
        let mut ans = 0;

        for i in 0..s.len() {
            if s.as_bytes()[i] == b'0' {
                zero += 1;
            } else {
                ones += 1;
            }

            if zero.abs_diff(ones) <= 1 {
                ans += 1;
            }
        }

        ans
    }
}