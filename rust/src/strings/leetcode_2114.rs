struct Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut ans = 0;
        for sentence in &sentences {
            let count = sentence.as_str()
                .split_ascii_whitespace()
                .filter(|w| !w.is_empty())
                .count();

            if count as i32 > ans { ans = count as i32; }
        }

        ans
    }
}