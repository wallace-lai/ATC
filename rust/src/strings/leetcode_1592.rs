struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let word = text.as_str()
            .split_ascii_whitespace()
            .filter(|w| !w.is_empty())
            .fold(0, |acc, _| acc + 1);

        let space = text.as_bytes()
            .iter()
            .filter(|c| **c == b' ')
            .count();

        let mut ans = String::with_capacity(text.len());
        if word < 2 {
            for w in text.as_str()
                .split_ascii_whitespace()
                .filter(|w| !w.is_empty()) {
                ans.push_str(w);
            }
            ans.push_str(&" ".repeat(space));
            return ans;
        }

        let between = space / (word - 1);
        let tail = space % (word - 1);
        for w in text.as_str()
            .split_ascii_whitespace()
            .filter(|w| !w.is_empty()) {
            if ans.len() > 0 {
                ans.push_str(&" ".repeat(between));
            }
            ans.push_str(w);
        }
        ans.push_str(&" ".repeat(tail));

        ans
    }
}