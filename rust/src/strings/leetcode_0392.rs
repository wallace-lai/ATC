struct Solution;

impl Solution {
    // s是否为t的子序列
    pub fn is_subsequence(s: String, t: String) -> bool {
        let slen = s.len();
        let tlen = t.len();
        let mut i = 0;
        let mut j = 0;

        while i < slen && j < tlen {
            if s.as_bytes()[i] == t.as_bytes()[j] {
                i += 1;
            }

            j += 1;
        }

        i == slen
    }
}