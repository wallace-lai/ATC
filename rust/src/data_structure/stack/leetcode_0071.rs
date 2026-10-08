struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stk = Vec::new();

        for p in path.split('/')
            .filter(|p| !p.is_empty()) {
            match p {
                "." => {},
                ".." => { stk.pop(); },
                _ => { stk.push(p.to_string()); }
            }
        }

        if stk.is_empty() { return "/".to_string(); }

        let mut ans = String::new();
        for p in stk {
            ans.push('/');
            ans.push_str(p.as_str());
        }
        ans
    }
}