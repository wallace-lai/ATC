struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut p = ["", ""];
        for (i, s) in text.split_whitespace()
            .filter(|s|!s.is_empty())
            .enumerate() {
            if i >= 2 {
                if p[0] == first && p[1] == second {
                    ans.push(s.to_string());
                }
            }

            p[0] = p[1];
            p[1] = s;
        }

        ans
    }
}