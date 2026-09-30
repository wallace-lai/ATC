struct Solution;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let b = s.as_bytes();
        let n = b.len();

        let mut max_zero_len = 0;
        let mut max_ones_len = 0;

        let mut zero_len = 0;
        let mut ones_len = 0;
        for i in 0..n {
            if b[i] == b'0' {
                if i == 0 || b[i - 1] == b'1' {
                    zero_len = 1;
                } else {
                    zero_len += 1;
                }
                max_zero_len = max_zero_len.max(zero_len);
            } else {
                if i == 0 || b[i - 1] == b'0' {
                    ones_len = 1;
                } else {
                    ones_len += 1;
                }
                max_ones_len = max_ones_len.max(ones_len);
            }
        }

        max_ones_len > max_zero_len
    }
}
