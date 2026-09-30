struct Solution;

impl Solution {
    pub fn check_two_chessboards(c1: String, c2: String) -> bool {
        let mut p0 = (c1.as_bytes()[0] - b'a') % 2;
        let mut p1 = (c1.as_bytes()[1] - b'0') % 2;
        let a = p0 ^ p1;

        p0 = (c2.as_bytes()[0] - b'a') % 2;
        p1 = (c2.as_bytes()[1] - b'0') % 2;
        let b = p0 ^ p1;

        a == b
    }
}