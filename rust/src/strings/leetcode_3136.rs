struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let b = word.as_bytes();
        let n = b.len();
        if n < 3 { return false; }

        let mut vowel = 0;
        let mut consonant = 0;
        for i in 0..n {
            if !b[i].is_ascii_alphabetic() &&
                !b[i].is_ascii_digit() {
                return false;
            }
            if b[i].is_ascii_alphabetic() {
                if "aeiouAEIOU".contains(b[i] as char) {
                    vowel += 1;
                } else {
                    consonant += 1;
                }
            }
        }

        vowel > 0 && consonant > 0
    }
}