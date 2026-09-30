struct Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut ans = 0;
        let b = s.as_bytes();
        let n = b.len();

        let mut table = 0;
        for i in 0..n {
            if b[i] == b'E' {
                table += 1;
                ans = std::cmp::max(ans, table);
            } else {
                table -= 1;
            }
        }

        ans
    }
}