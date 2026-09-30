struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut lower_count = [0; 26];
        let mut upper_count = [0; 26];
        for c in s.chars() {
            if c.is_uppercase() {
                let idx = (c as u8 - b'A') as usize;
                upper_count[idx] += 1;
            } else {
                let idx = (c as u8 - b'a') as usize;
                lower_count[idx] += 1;
            }
        }

        let mut ans = String::new();
        for i in (0..upper_count.len()).rev() {
            if upper_count[i] > 0 && lower_count[i] > 0 {
                let char = (b'A' + i as u8) as char;
                ans.push(char);
                break;
            }
        }

        ans
    }
}