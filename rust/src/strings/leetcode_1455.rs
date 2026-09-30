struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut ans = -1;
        for (i, s) in sentence.as_str()
            .split_ascii_whitespace()
            .filter(|w|!w.is_empty())
            .enumerate() {
            if s.starts_with(&search_word) {
                ans = i as i32 + 1;
                break;
            }
        }

        ans
    }
}