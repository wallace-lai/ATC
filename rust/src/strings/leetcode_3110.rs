struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut ans = 0;
        let b = s.as_bytes();
        let n = b.len();

        for i in 1..n {
            let d = b[i] as i32 - b[i - 1] as i32;
            ans += if d < 0 { 0 - d } else { d };
        }

        ans
    }
}