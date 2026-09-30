struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut v = Vec::with_capacity(s.len());

        let mut count = 0;
        for &c in s.as_bytes().iter().rev() {
            if c == b'-' { continue; }
            v.push(c.to_ascii_uppercase());
            count += 1;
            if count == k {
                v.push(b'-');
                count = 0;
            }
        }
        if v.len() == 0 { return "".to_string(); }

        if v[v.len() - 1] == b'-' { v.pop(); }
        v.reverse();

        unsafe { String::from_utf8_unchecked(v) }
    }
}