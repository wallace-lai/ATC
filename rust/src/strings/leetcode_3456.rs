struct Solution;

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let mut len = 0;
        for (i, c) in s.as_bytes().iter().enumerate() {
            len += 1;
            if i == s.len() - 1 || *c != s.as_bytes()[i + 1] {
                if len == k { return true; }
                len = 0;
            }
        }

        false
    }
} 