struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        v.extend_from_slice(s.as_bytes());

        for i in 0..v.len() {
            if i & 1 == 1 {
                v[i] = v[i - 1] + v[i] - b'0';
            }
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}