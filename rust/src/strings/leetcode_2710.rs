struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let b = num.as_bytes();
        let n = b.len();
        let mut end = n - 1;

        while b[end] == b'0' {
            end -= 1;
        }

        let mut v = Vec::with_capacity(n);
        v.extend_from_slice(&b[0..=end]);

        unsafe { String::from_utf8_unchecked(v) }
    }
}