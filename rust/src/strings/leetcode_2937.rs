struct Solution;

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let n1 = s1.len();
        let n2 = s2.len();
        let n3 = s3.len();

        let mut i = 0;
        while i < n1 && i < n2 && i < n3 {
            if s1.as_bytes()[i] != s2.as_bytes()[i] ||
                s2.as_bytes()[i] != s3.as_bytes()[i] {
                break;
            }

            i += 1;
        }

        if i == 0 { return -1; }
        (n1 + n2 + n3 - 3 * i) as i32
    }
}