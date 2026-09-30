struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut ans = 0;
        let b = s.as_bytes();
        let n = b.len();

        for i in 1..n {
            if b[i].to_ascii_lowercase() !=
                b[i - 1].to_ascii_lowercase() {
                ans += 1;
            }
        }

        ans
    }
}