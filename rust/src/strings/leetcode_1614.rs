struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut ans = 0;
        let mut v = Vec::with_capacity(s.len());

        for &i in s.as_bytes() {
            if i == b'(' {
                v.push(i);
                ans = ans.max(v.len());
            } else if i == b')' {
                v.pop();
            }
        }

        ans as i32
    }
}