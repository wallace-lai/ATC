struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in &words {
            let is_palindrome = {
                let mut ans = true;
                let mut left = 0;
                let mut right = word.len() as i32 - 1;
                while left < right {
                    if word.as_bytes()[left as usize] !=
                        word.as_bytes()[right as usize] {
                        ans = false;
                        break;
                    }

                    left += 1;
                    right -= 1;
                }

                ans
            };

            if is_palindrome { return word.clone(); }
        }

        "".to_string()
    }
}