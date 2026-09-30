struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut ones = 0;
        let b = s.as_bytes();
        let n = b.len();

        for i in 0..n {
            if b[i] == b'1' &&
                (i == 0 || b[i - 1] != b'1') {
                ones += 1;
            }
        }

        println!("ones is {ones}");
        ones <= 1
    }
}