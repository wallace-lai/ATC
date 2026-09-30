struct Solution;

impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        let bstr = s.as_bytes();
        let mut i = s.len() as i32 - 1;
        while i >= 0 {
            if bstr[i as usize] == b'a' ||
                bstr[i as usize] == b'e' ||
                bstr[i as usize] == b'i' ||
                bstr[i as usize] == b'o' ||
                bstr[i as usize] == b'u' {
                i -= 1;
            } else {
                break;
            }
        }

        if i < 0 { return "".to_string(); }

        let ans = &s[0..=i as usize];
        ans.to_string()
    }
}