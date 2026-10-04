struct Solution;

impl Solution {
    pub fn resulting_string(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut stk = Vec::with_capacity(n);
        
        for &c in s.as_bytes().iter() {
            if stk.len() > 0 {
                let &t = stk.last().unwrap();
                if (t - b'a' + 1) % 26 == c - b'a' ||
                    (c - b'a' + 1) % 26 == t - b'a' {
                    stk.pop();
                    continue;
                }
            }

            stk.push(c);
        }

        unsafe { String::from_utf8_unchecked(stk) }
    }
}
