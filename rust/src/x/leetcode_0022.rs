struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(s: &mut String, left: i32, right: i32, ans: &mut Vec<String>) {
            if left == 0 && right == 0 {
                ans.push(s.clone());
                return;
            }

            if left > 0 {
                s.push('(');
                dfs(s, left - 1, right, ans);
                s.pop();
            }
            if right > 0 && right > left {
                s.push(')');
                dfs(s, left, right - 1, ans);
                s.pop();
            }
        }

        let mut s = String::with_capacity(n as usize * 2);
        let mut ans = Vec::new();
        dfs(&mut s, n, n, &mut ans);
        ans
    }
}