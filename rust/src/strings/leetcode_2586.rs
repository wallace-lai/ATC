struct Solution;

impl Solution {
    pub fn is_vowel(c: u8) -> bool {
        c == b'a' || c == b'e' || c == b'i' ||
        c == b'o' || c == b'u'
    }

    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        for i in left..=right {
            let b = words[i as usize].as_bytes();
            if Self::is_vowel(b[0]) && Self::is_vowel(b[b.len() - 1]) {
                ans += 1;
            }
        }

        ans
    }
}