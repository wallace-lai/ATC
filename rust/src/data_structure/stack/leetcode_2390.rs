struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ans = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c == '*' { ans.pop(); }
            else { ans.push(c); }
        }
        String::from_iter(ans)
    }
}