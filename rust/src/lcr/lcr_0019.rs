struct Solution;

impl Solution {
    // WA
    // pub fn valid_palindrome(s: String) -> bool {
    //     let str = s.as_bytes();
    //     let mut count = [0; 26];
    //     for &c in str {
    //         let idx = (c - b'a') as usize;
    //         count[idx] += 1;
    //     }

    //     let mut left = 0;
    //     let mut right = s.len() as i32 - 1;
    //     while left < right {
    //         if str[left as usize] != str[right as usize] {
    //             break;
    //         }

    //         let idx = (str[left as usize] - b'a') as usize;
    //         count[idx] -= 2;
    //         left += 1;
    //         right -= 1;
    //     }
    //     if right >= left { return true; }

        
    //     if right - left + 1 > 2 { false } else { true }
    // }

    pub fn check(s: &String, mut i: i32, mut j: i32) -> bool {
        while i < j {
            if s.as_bytes()[i as usize] != s.as_bytes()[j as usize] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let mut i = 0;
        let mut j = s.len() as i32 - 1;
        while i < j {
            if s.as_bytes()[i as usize] ==
                s.as_bytes()[j as usize] {
                i += 1;
                j -= 1;
            } else {
                return Self::check(&s, i, j - 1) ||
                    Self::check(&s, i + 1, j);
            }
        }

        true
    }
}