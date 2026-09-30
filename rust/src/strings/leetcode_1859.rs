struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut v: Vec<&str> = Vec::with_capacity(10);
        for w in s.as_str()
            .split_ascii_whitespace()
            .filter(|w| !w.is_empty()) {
            v.push(w);
        }
        v.sort_unstable_by_key(|w| w.as_bytes()[w.len() - 1]);


        let mut ans = Vec::with_capacity(s.len());
        for (i, w) in v.iter().enumerate() {
            if i > 0 { ans.push(b' '); }
            ans.extend_from_slice(&w.as_bytes()[..(w.len() - 1)]);
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}