struct Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        // u8 --> u8
        let mut map = [-1_i32; 26];
        let mut pos = 0;
        for &c in key.as_bytes() {
            if c.is_ascii_lowercase() {
                let idx = (c - b'a') as usize;
                if map[idx] < 0 {
                    map[idx] = pos;
                    pos += 1;
                }
            }
        }

        let mut decode = vec![0_u8; message.len()];
        for (i, &c) in message.as_bytes().iter().enumerate() {
            if c.is_ascii_lowercase() {
                let idx = (c - b'a') as usize;
                decode[i] = map[idx] as u8 + b'a';
            } else {
                decode[i] = c;
            }
        }

        String::from_utf8(decode).unwrap()
    }
}