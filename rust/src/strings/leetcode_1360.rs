struct Solution;

impl Solution {
    pub fn is_leap_year(year: i32) -> bool {
        (year % 400 == 0) ||
        (year % 4 == 0 && year % 100 != 0)
    }

    pub fn days_in_month(year: i32, month: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 0
        }
    }

    pub fn days_from_epoch(year: i32, month: i32, day: i32) -> i32 {
        let mut days = 0;

        // 年份在1971到2100之间
        for y in 1970..year {
            days += if Self::is_leap_year(y) { 366 } else { 365 };
        }

        for m in 1..month {
            days += Self::days_in_month(year, m);
        }

        days += day;

        days
    }

    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let mut b = date1.as_bytes();
        let y1 = (b[0] - b'0') as i32 * 1000 +
            (b[1] - b'0') as i32 * 100 +
            (b[2] - b'0') as i32 * 10 +
            (b[3] - b'0') as i32;
        
        let m1 = (b[5] - b'0') as i32 * 10 +
            (b[6] - b'0') as i32;
        
        let d1 = (b[8] - b'0') as i32 * 10 +
            (b[9] - b'0') as i32;
        
        b = date2.as_bytes();
        let y2 = (b[0] - b'0') as i32 * 1000 +
            (b[1] - b'0') as i32 * 100 +
            (b[2] - b'0') as i32 * 10 +
            (b[3] - b'0') as i32;
        
        let m2 = (b[5] - b'0') as i32 * 10 +
            (b[6] - b'0') as i32;
        
        let d2 = (b[8] - b'0') as i32 * 10 +
            (b[9] - b'0') as i32;
        
        if (y2, m2, d2) > (y1, m1, d1) {
            Self::days_from_epoch(y2, m2, d2) -
            Self::days_from_epoch(y1, m1, d1)
        } else {
            Self::days_from_epoch(y1, m1, d1) -
            Self::days_from_epoch(y2, m2, d2)
        }
    }
}
