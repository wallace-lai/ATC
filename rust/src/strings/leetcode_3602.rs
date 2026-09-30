struct Solution;

impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let mut v1 = Vec::new();
        let mut tmp = n * n;
        while tmp > 0 {
            let r = tmp % 16;
            if r < 10 {
                v1.push(r as u8 + b'0');
            } else {
                v1.push(r as u8 - 10 + b'A');
            }
            tmp /= 16;
        }
        v1.reverse();

        let mut v2 = Vec::new();
        tmp = n * n * n;
        while tmp > 0 {
            let r = tmp % 36;
            if r < 10 {
                v2.push(r as u8 + b'0');
            } else {
                v2.push(r as u8 - 10 + b'A');
            }
            tmp /= 36;
        }
        v2.reverse();

        let mut v = Vec::with_capacity(v1.len() + v2.len());
        v.extend(v1.into_iter());
        v.extend(v2.into_iter());
        unsafe { String::from_utf8_unchecked(v) }
    }
}