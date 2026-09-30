struct Solution;

impl Solution {
    pub fn parse_time(s: &str) -> Option<(i32, i32, i32)> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return None;
        }

        let h = parts[0].parse::<i32>().ok()?;
        let m = parts[1].parse::<i32>().ok()?;
        let s = parts[2].parse::<i32>().ok()?;
        if h < 24 && m < 60 && s < 60 {
            Some((h, m, s))
        } else {
            None
        }
    }

    pub fn seconds_between_times(start_time: String, end_time: String) -> i32 {
        let mut ans = 0;

        let start = Self::parse_time(&start_time).unwrap();
        let end = Self::parse_time(&end_time).unwrap();
        ans += (end.0 - start.0) * 3600;
        ans += (end.1 - start.1) * 60;
        ans += end.2 - start.2;

        ans
    }
}