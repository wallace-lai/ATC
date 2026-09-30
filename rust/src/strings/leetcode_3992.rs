struct Solution;

impl Solution {
    pub fn rearrange_string(s: String, _: char, y: char) -> String {
        let mut ans: Vec<u8> = vec![0; s.len()];
        let mut i = 0;
        let mut j = s.len() - 1;

        for &c in s.as_bytes() {
            if c as char == y {
                ans[i] = c;
                i += 1;
            } else {
                ans[j] = c;
                j -= 1;
            }
        }
            
        unsafe { String::from_utf8_unchecked(ans) }
    }
}