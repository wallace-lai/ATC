struct Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut ans = 0;
        for word in &words {
            if s.starts_with(word.as_str()) {
                ans += 1;
            }
        }
        ans
    }
}