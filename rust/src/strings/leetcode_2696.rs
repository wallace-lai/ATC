struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut v = Vec::with_capacity(n);

        for i in 0..n {
            if b[i] == b'B' &&
                v.len() > 0 &&
                v.last().unwrap() == &b'A' {
                v.pop();
                continue;
            }
            if b[i] == b'D' &&
                v.len() > 0 &&
                v.last().unwrap() == &b'C' {
                v.pop();
                continue;
            }

            v.push(b[i]);
        }

        v.len() as i32
    }
}