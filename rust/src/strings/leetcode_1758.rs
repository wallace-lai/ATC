struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut ans = i32::MAX;
        let b = s.as_bytes();
        let n = b.len();

        let mut x = b'0';
        let mut min = 0;
        for i in 0..n {
            if b[i] != x { min += 1; }
            x = if x == b'0' { b'1' } else { b'0' };
        }
        ans = ans.min(min);

        x = b'1';
        min = 0;
        for i in 0..n {
            if b[i] != x { min += 1; }
             x = if x == b'0' { b'1' } else { b'0' };
        }
        ans = ans.min(min);

        ans
    }
}