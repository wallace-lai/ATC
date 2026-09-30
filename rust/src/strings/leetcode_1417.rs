struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut alpha = Vec::new();
        let mut digit = Vec::new();
        for &c in s.as_bytes() {
            if c.is_ascii_alphabetic() {
                alpha.push(c);
            } else {
                digit.push(c);
            }
        }

        if alpha.len() > digit.len() + 1 ||
            digit.len() > alpha.len() + 1 {
            return "".to_string();
        }

        let mut v = Vec::with_capacity(s.len());
        if alpha.len() > digit.len() {
            while digit.len() > 0 {
                v.push(alpha.pop().unwrap());
                v.push(digit.pop().unwrap());
            }
        } else {
            while alpha.len() > 0 {
                v.push(digit.pop().unwrap());
                v.push(alpha.pop().unwrap());
            }
        }
        if let Some(val) = alpha.pop() {
            v.push(val);
        }
        if let Some(val) = digit.pop() {
            v.push(val);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}