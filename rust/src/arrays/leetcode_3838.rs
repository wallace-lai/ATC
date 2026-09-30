struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(words.len());
        for word in words.iter() {
            let mut sum = 0;
            for &c in word.as_bytes() {
                let idx = (c - b'a') as usize;
                sum += weights[idx];
            }

            let new_char = (25 - sum % 26) as u8 + b'a';
            v.push(new_char);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}