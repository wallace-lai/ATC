struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();

        let ones = b.iter().filter(|&c| *c == b'1').count();
        let mut v = vec![b'0'; n];
        for i in 0..(ones - 1) {
            v[i] = b'1';
        }
        v[n - 1] = b'1';

        unsafe { String::from_utf8_unchecked(v) }
    }
}