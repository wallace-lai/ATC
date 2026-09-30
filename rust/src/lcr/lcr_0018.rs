struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v: Vec<char> = s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| {
                if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect();
        
        if v.len() < 2 { return true; }

        // println!("v is {:?}", v);

        let mut left = 0;
        let mut right = v.len() - 1;
        while left < right {
            if v[left] != v[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}