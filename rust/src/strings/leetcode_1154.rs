struct Solution;

impl Solution {
    const LEAP_YEAR: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const AVER_YEAR: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    pub fn day_of_year(date: String) -> i32 {
        let b = date.as_bytes();
        let year = (b[0] - b'0') as i32 * 1000 +
            (b[1] - b'0') as i32 * 100 +
            (b[2] - b'0') as i32 * 10 +
            (b[3] - b'0') as i32;
        
        let month = (b[5] - b'0') as i32 * 10 +
            (b[6] - b'0') as i32;
        
        let day = (b[8] - b'0') as i32 * 10 +
            (b[9] - b'0') as i32;

        let is_leap = (year % 4 == 0 && year % 100 != 0) ||
            (year % 400 == 0);
        
        let mut ans = 0;
        for i in 0..(month - 1) {
            ans += if is_leap {
                Self::LEAP_YEAR[i as usize]
            } else {
                Self::AVER_YEAR[i as usize]
            };
        }
        ans += day;

        ans
    }
}