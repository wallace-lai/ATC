struct Solution;

impl Solution {
    // 直接计算()对答案的最终贡献
    pub fn score_of_parentheses(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut d = 0;
        let mut ans = 0;

        for i in 0..n {
            if b[i] == b'(' {
                d += 1;
            } else {
                if b[i - 1] == b'(' {
                    ans += 2i32.pow(d - 1);
                }
                d -= 1;
            }
        }

        ans
    }
}
