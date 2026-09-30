struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut v = time.as_bytes().to_vec();
        // assert_eq!(v.len(), 5);
        for i in 0..v.len() {
            if v[i] == b'?' {
                match i {
                    0 => {
                        v[i] = if v[i + 1] != b'?' && v[i + 1] > b'3' {
                            b'1'
                        } else {
                            b'2'
                        };
                    },
                    1 => {
                        v[i] = if v[i - 1] == b'2' {
                            b'3'
                        } else {
                            b'9'
                        };
                    },
                    3 => {
                        v[i] = b'5';
                    },
                    4 => {
                        v[i] = b'9';
                    },
                    _ => {}
                }
            }
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}