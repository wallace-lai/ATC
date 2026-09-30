struct Solution;

impl Solution {
    pub fn is_palindrome(s: &str) -> bool {
        let mut left = 0;
        let mut right = s.len() as i32 - 1;
        while left < right {
            if s.as_bytes()[left as usize] !=
                s.as_bytes()[right as usize] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }

    pub fn remove_palindrome_sub(s: String) -> i32 {
        if Self::is_palindrome(&s) { 1 } else { 2 }
    }
}