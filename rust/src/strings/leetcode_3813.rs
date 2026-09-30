struct Solution;

impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut v = 0;
        let mut c = 0;

        for &i in s.as_bytes() {
            if i.is_ascii_alphabetic() {
                if i == b'a' ||
                    i == b'e' ||
                    i == b'i' ||
                    i == b'o' ||
                    i == b'u' {
                    v += 1;
                } else {
                    c += 1;
                }
            }
        }

        if c == 0 { return 0; }
        v / c
    }
}