struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut valid = vec![0; n];
        let mut stack = Vec::with_capacity(s.len());

        for (i, &c) in b.iter().enumerate() {
            if c == b'(' {
                stack.push(i);
                continue;
            }
            if stack.len() > 0 {
                let &p = stack.last().unwrap();
                if b[p] == b'(' {
                    valid[p] = 1;
                    valid[i] = 1;
                    stack.pop();
                }
            }
        }

        let mut cnt = 0;
        let mut ans = 0;
        for v in valid {
            if v == 0 {
                cnt = 0;
            } else {
                cnt += 1;
                ans = ans.max(cnt);
            }
        }

        ans
    }
}