struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut n = 0;
        let mut v: Vec<u8> = Vec::with_capacity(number.len());
        for (i, c) in number.as_bytes()
            .iter()
            .filter(|c| c.is_ascii_digit())
            .enumerate() {
            n += 1;
            if i > 0 && i % 3 == 0 { v.push(b'-'); }
            v.push(*c);
        }
        if n % 3 == 1 {
            let len = v.len();
            let tmp = v[len - 2];
            v[len - 2] = v[len - 3];
            v[len - 3] = tmp;
        }
        unsafe { String::from_utf8_unchecked(v) }
    }
}