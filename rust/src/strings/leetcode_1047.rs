struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        for &c in s.as_bytes() {
            if let Some(top) = v.last() && *top == c {
                v.pop();
                continue;
            }

            v.push(c);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}