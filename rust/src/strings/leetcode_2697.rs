struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut v = Vec::with_capacity(s.len());
        v.extend_from_slice(s.as_bytes());

        let mut left = 0;
        let mut right = v.len() as i32 - 1;
        while left < right {
            if v[left as usize] != v[right as usize] {
                if v[left as usize] > v[right as usize] {
                    v[left as usize] = v[right as usize];
                } else {
                    v[right as usize] = v[left as usize];
                }
            }

            left += 1;
            right -= 1;
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}