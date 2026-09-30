struct Solution;

impl Solution {
    pub fn is_balanced(slice: &[u8]) -> bool {
        let head = &slice[0..slice.len() / 2];
        let tail = &slice[slice.len() / 2..];
        if head.iter().all(|c| *c == b'0') &&
            tail.iter().all(|c| *c == b'1') {
            return true;
        }

        false
    }

    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let n = s.len();
        let b = s.as_bytes();
        let mut ans = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                let slice = &b[i..=j];
                if slice.len() & 1 == 1 { continue; }
                if Self::is_balanced(slice) && slice.len() > ans {
                    ans = slice.len();
                }
            }
        }

        ans as i32
    }
}