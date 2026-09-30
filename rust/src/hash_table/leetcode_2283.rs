struct Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = [0u8; 16];
        for &c in num.as_bytes() {
            let idx = (c - b'0') as usize;
            count[idx] += 1;
        }

        for (i, &c) in num.as_bytes().iter().enumerate() {
            let cnt = c - b'0';
            if cnt != count[i] {
                return false;
            }
        }

        true
    }
}