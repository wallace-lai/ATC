struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn build_key(str: &[u8], key: &mut [i32; 26]) {
        for &c in str {
            let idx = (c - b'a') as usize;
            key[idx] += 1;
        }
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s == t { return false; }
        let mut skey = [0; 26];
        let mut tkey = [0; 26];

        Self::build_key(s.as_bytes(), &mut skey);
        Self::build_key(t.as_bytes(), &mut tkey);
        if skey != tkey { return false; }

        true

    }
}