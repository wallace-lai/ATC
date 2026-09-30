struct Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut ans = b'a';
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            if count[idx] == 1 {
                ans = c;
                break;
            }
            count[idx] += 1;
        }

        ans as char
    }
}