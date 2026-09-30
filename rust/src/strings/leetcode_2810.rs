struct Solution;

impl Solution {
    pub fn final_string(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut v = Vec::with_capacity(n);

        for i in 0..n {
            if b[i] == b'i' && v.len() > 0 {
                v.reverse();
                continue;
            }
            v.push(b[i]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}