struct Solution;

impl Solution {
    pub fn largest_even(s: String) -> String {
        let mut i = s.len() as i32 - 1;
        while i >= 0 {
            if s.as_bytes()[i as usize] != b'2' {
                i -= 1;
            } else {
                break;
            }
        }

        if i < 0 { return String::new(); }

        let ans = &s.as_str()[0..=i as usize];
        ans.to_string()
    }
}