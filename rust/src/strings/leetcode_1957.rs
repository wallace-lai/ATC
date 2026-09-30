struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        if s.len() <= 2 { return s; }
        let b = s.as_bytes();
        let n = b.len();

        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        let mut x = b[0];
        let mut y = b[1];
        v.push(x);
        v.push(y);

        for i in 2..n {
            if x == y && b[i] == x { continue; }
            v.push(b[i]);
            x = y;
            y = b[i];
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}