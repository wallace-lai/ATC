struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let len = s.len();
        if len == 1 {
            return if s > target { s } else { String::new() };
        }

        let mut count = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        let mut odd = String::new();
        for i in 0..26 {
            if count[i] & 1 == 1 {
                if !odd.is_empty() {
                    return String::new();
                }
                odd = ((b'a' + i as u8) as char).to_string();
            }
            count[i] /= 2;
        }

        let mut pre = String::new();
        for i in 0..len / 2 {
            let mut found = false;
            for j in 0..26 {
                if count[j] == 0 { continue; }
                count[j] -= 1;

                let mut left = pre.clone();
                left.push((b'a' + j as u8) as char);
                for k in (0..26).rev() {
                    for _ in 0..count[k] {
                        left.push((b'a' + k as u8) as char);
                    }
                }

                let mut palindrome = left.clone();
                palindrome.push_str(&odd);
                let reverse_left: String = left.chars().rev().collect();
                palindrome.push_str(&reverse_left);
                
                if palindrome > target {
                    pre.push((b'a' + j as u8) as char);
                    found = true;
                    break;
                } else {
                    count[j] += 1;
                }
            }
            if !found { return String::new(); }

            if pre.as_bytes()[i] > target.as_bytes()[i] {
                let mut left = pre.clone();
                for j in 0..26 {
                    for _ in 0..count[j] {
                        left.push((b'a' + j as u8) as char);
                    }
                }
                let mut palindrome = left.clone();
                palindrome.push_str(&odd);
                let reversed_left: String = left.chars().rev().collect();
                palindrome.push_str(&reversed_left);
                return palindrome;
            }
        }

        let mut ans = pre.clone();
        ans.push_str(&odd);
        let reversed_pre: String = pre.chars().rev().collect();
        ans.push_str(&reversed_pre);
        ans
    }
}