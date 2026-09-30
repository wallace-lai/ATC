struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = String::new();
        let mut stack = Vec::with_capacity(s.len());

        for &c in s.as_bytes() {
            if c == b')' { stack.pop(); }
            if !stack.is_empty() { ans.push(c as char); }
            if c == b'(' { stack.push(c); }
        }

        ans
    }
}