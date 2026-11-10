struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        
        fn dfs(n: usize, l: usize, r: usize, t: &mut String, ans: &mut Vec<String>) {
            // 左括号已用完，剩下只能全部填右括号
            if l == n {
                let old_len = t.len();
                t.extend(std::iter::repeat(')').take(n - r));
                ans.push(t.clone());
                t.truncate(old_len);
                return;
            }

            // 填左括号
            if l < n {
                t.push('(');
                dfs(n, l + 1, r, t, ans);
                t.pop();
            }

            // 填右括号
            if l > r {
                t.push(')');
                dfs(n, l, r + 1, t, ans);
                t.pop();
            }
        }

        let n = n as usize;
        let mut t = String::with_capacity(n * 2);
        let mut ans = Vec::new();

        dfs(n, 0, 0, &mut t, &mut ans);
        ans
    }
}