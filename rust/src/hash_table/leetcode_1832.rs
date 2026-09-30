struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut count = [0; 26];
        for &c in sentence.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        count.iter().all(|cnt| *cnt > 0)
    }
}