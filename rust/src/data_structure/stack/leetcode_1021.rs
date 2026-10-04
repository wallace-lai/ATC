struct Solution;

impl Solution {
    // 法一
    // pub fn remove_outer_parentheses(s: String) -> String {
    //     let mut ans = String::new();
    //     let mut stack = Vec::with_capacity(s.len());

    //     for &c in s.as_bytes() {
    //         if c == b')' { stack.pop(); }
    //         if !stack.is_empty() { ans.push(c as char); }
    //         if c == b'(' { stack.push(c); }
    //     }

    //     ans
    // }

    // 法二：只记录深度，删除深度为0的那层括号即可
    pub fn remove_outer_parentheses(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut d = 0;  // 括号的深度
        let mut ans = String::with_capacity(n);

        for i in 0..n {
            if b[i] == b'(' {
                if d > 0 { ans.push(b[i] as char); }
                d += 1;
            } else {
                d -= 1;
                if d > 0 { ans.push(b[i] as char); }
            }
        }

        ans
    }
}