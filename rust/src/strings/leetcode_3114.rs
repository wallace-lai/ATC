struct Solution;

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut v = s.as_bytes().to_vec();

        if v[0] == b'?' {
            if v[1] == b'?' || v[1] < b'2' {
                v[0] = b'1';
            } else {
                v[0] = b'0';
            }
        }
        if v[1] == b'?' {
            if v[0] == b'0' {
                v[1] = b'9';
            } else {
                v[1] = b'1';
            }
        }
        if v[3] == b'?' { v[3] = b'5'; }
        if v[4] == b'?' { v[4] = b'9'; }

        unsafe { String::from_utf8_unchecked(v) }
    }
}