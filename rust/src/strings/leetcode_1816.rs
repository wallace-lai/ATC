struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        for (i, w) in s.as_str()
            .split_ascii_whitespace()
            .filter(|w| !w.is_empty())
            .enumerate() {
            if i >= k as usize { break; }
            if i > 0 { v.push(b' '); }
            v.extend_from_slice(w.as_bytes());
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}