struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        let b = s.as_bytes();
        let n = b.len();

        for i in 0..n {
            if b[i].is_ascii_digit() &&
                v.len() > 0 &&
                v.last().unwrap().is_ascii_lowercase() {
                v.pop();
                continue;
            }
            v.push(b[i]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}