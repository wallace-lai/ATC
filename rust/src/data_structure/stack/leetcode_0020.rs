struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let b = s.as_bytes();
        let n = b.len();

        let is_pair = |a: u8, b: u8| -> bool {
            if (a == b'(' && b == b')') ||
                (a == b'{' && b == b'}') ||
                (a == b'[' && b == b']') {
                return true;
            }
            false
        };

        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            match b[i] {
                b'(' | b'{' | b'[' => { v.push(b[i]); },
                b')' | b'}' | b']' => { 
                    if v.len() > 0 && is_pair(*v.last().unwrap(), b[i]) {
                        v.pop();
                    } else {
                        v.push(b[i]);
                    }
                },
                _ => {}
            }
        }

        v.len() == 0
    }
}