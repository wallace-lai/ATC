struct Solution;

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut v = s.as_bytes().to_vec();
        let k = k as usize % v.len();
        let b = s.as_bytes();

        for i in 0..v.len() {
            let idx = (i + k) % v.len();
            v[i] = b[idx];
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}