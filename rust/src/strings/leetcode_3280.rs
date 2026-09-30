struct Solution;

impl Solution {
    pub fn parse_date(s: &str) -> Option<(i32, i32, i32)> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 { return None; }

        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<i32>().ok()?;
        let day = parts[2].parse::<i32>().ok()?;
        Some((year, month, day))
    }

    pub fn convert_date_to_binary(date: String) -> String {
        let (y, m, d) = Self::parse_date(&date).unwrap();
        format!("{:b}-{:b}-{:b}", y, m, d)
    }
}