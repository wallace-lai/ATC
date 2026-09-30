struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let pos = word.find(ch);
        if let Some(idx) = pos {
            let mut v: Vec<u8> = Vec::with_capacity(word.len());
            v.extend_from_slice(&word.as_bytes()[0..=idx]);
            v.reverse();
            v.extend_from_slice(&word.as_bytes()[idx + 1..]);
            return unsafe { String::from_utf8_unchecked(v) };
        }

        word
    }
}