struct Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let len: usize = words.iter().map(|w| w.len()).sum();
        let mut wvec: Vec<u8> = Vec::with_capacity(len);
        let svec = s.as_bytes().to_vec();

        for i in 0..words.len() {
            wvec.extend_from_slice(words[i].as_bytes());
            if svec == wvec {
                return true;
            } else if wvec.len() >= svec.len() {
                return false;
            }
        }

        false
    }
}