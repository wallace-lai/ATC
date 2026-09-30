struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut last_idx_a = -1;
        let mut first_idx_b = -1;
        let b = s.as_bytes();
        for i in 0..b.len() {
            if b[i] == b'a' { last_idx_a = i as i32; }
            if b[i] == b'b' && first_idx_b == -1 {
                first_idx_b = i as i32;
            }
        }

        if last_idx_a == -1 || first_idx_b == -1 { return true; }
        last_idx_a < first_idx_b
    }
}