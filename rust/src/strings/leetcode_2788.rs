struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut v = Vec::new();
        for word in &words {
            for w in word.split(separator)
                .filter(|w| !w.is_empty()) {
                // println!("w is {:?}", w);
                v.push(w.to_string());
            }
        }

        v
    }
}