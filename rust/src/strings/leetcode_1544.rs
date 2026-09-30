struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        
        for i in 0..n {
            if v.len() > 0 {
                let &top = v.last().unwrap();
                if top.eq_ignore_ascii_case(&b[i]) {
                    if (top.is_ascii_lowercase() && b[i].is_ascii_uppercase()) ||
                        (top.is_ascii_uppercase() && b[i].is_ascii_lowercase()) {
                        v.pop();
                        continue;
                    }
                }
            }

            v.push(b[i]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}