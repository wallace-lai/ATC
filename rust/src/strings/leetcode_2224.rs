struct Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let curr_minutes =
            (current.as_bytes()[0] - b'0') as i32 * 10 * 60 +
            (current.as_bytes()[1] - b'0') as i32 * 60 +
            (current.as_bytes()[3] - b'0') as i32 * 10 +
            (current.as_bytes()[4] - b'0') as i32;

        let corr_minutes =
            (correct.as_bytes()[0] - b'0') as i32 * 10 * 60 +
            (correct.as_bytes()[1] - b'0') as i32 * 60 +
            (correct.as_bytes()[3] - b'0') as i32 * 10 +
            (correct.as_bytes()[4] - b'0') as i32;

        let mut ans = 0;
        let mut n = corr_minutes - curr_minutes;

        ans += n / 60;
        n %= 60;

        ans += n / 15;
        n %= 15;

        ans += n / 5;
        n %= 5;

        ans + n
    }
}