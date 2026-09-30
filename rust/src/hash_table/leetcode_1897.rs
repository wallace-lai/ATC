struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let len = words.len() as i32;
        let mut count = [0; 26];
        for word in words {
            for &c in word.as_bytes() {
                let idx = (c - b'a') as usize;
                count[idx] += 1;
            }
        }

        for cnt in count {
            if cnt % len != 0 {
                return false;
            }
        }

        true
    }
}