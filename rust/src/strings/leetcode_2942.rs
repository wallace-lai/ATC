struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut v = Vec::with_capacity(words.len());
        for (i, w) in words.iter().enumerate() {
            if w.contains(x) {
                v.push(i as i32);
            }
        }

        v
    }
}