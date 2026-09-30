struct Solution;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let n = s.len();
        if n < 2 { return String::new(); }

        let mut max_pos = 0;
        let mut max_len = 0;
        let bytes = s.as_bytes();

        for i in 0..n {
            let mut lower: u32 = 0;
            let mut upper: u32 = 0;

            for j in i..n {
                if bytes[j].is_ascii_lowercase() {
                    lower |= 1 << (bytes[j] - b'a') as u32;
                } else {
                    upper |= 1 << (bytes[j] - b'A') as u32;
                }

                if lower == upper && j - i + 1 > max_len {
                    max_pos = i;
                    max_len = j - i + 1;
                }
            }
        }

        let substr: Vec<u8> = Vec::from(&bytes[max_pos..(max_pos + max_len)]);
        String::from_utf8(substr).expect("convert failed")
    }
}