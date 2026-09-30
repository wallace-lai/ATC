struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev: i32 = 0;

        for (i, token) in s.as_str()
            .split_ascii_whitespace()
            .filter(|t| { 
                !t.is_empty() &&
                t.as_bytes()[0].is_ascii_digit()
            })
            .enumerate() {
            if i == 0 { prev = token.parse().unwrap(); continue; }
            let curr: i32 = token.parse().unwrap();
            if curr <= prev { return false; }
            prev = curr;
        }

        true
    }
}