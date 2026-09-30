struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut v: Vec<u8> = s.as_bytes().iter().cloned().collect();

        let n = v.len() as i32;
        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            while left < n && !v[left as usize].is_ascii_alphabetic() {
                left += 1;
            }
            while right >= 0 && !v[right as usize].is_ascii_alphabetic() {
                right -= 1;
            }
            if left < right {
                let tmp = v[left as usize];
                v[left as usize] = v[right as usize];
                v[right as usize] = tmp;

                left += 1;
                right -= 1;
            }
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}