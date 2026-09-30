struct Solution;

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut ans = 0;
        let b = s.as_bytes();
        let mut i = 0;

        while i < b.len() {
            if b[i] == b'X' {
                ans += 1;
                i += 3;
                continue;
            }
            i += 1;
        }

        ans
    }
}