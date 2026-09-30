struct Solution;

impl Solution {
    const SPECIAL_CHARS: &'static str = "!@#$%^&*()-+";

    pub fn strong_password_checker_ii(password: String) -> bool {
        let b = password.as_bytes();
        let n = b.len();
        // 1. 至少含义8个字符
        if n < 8 { return false; }

        let mut lower = 0;
        let mut upper = 0;
        let mut digit = 0;
        let mut special = 0;

        for i in 0..n {
            // 6. 不包含2个连续相同字符
            if i + 1 < n && b[i] == b[i + 1] { return false; }

            if b[i].is_ascii_lowercase() { lower += 1; }
            else if b[i].is_ascii_uppercase() { upper += 1; }
            else if b[i].is_ascii_digit() { digit += 1; }
            else if Self::SPECIAL_CHARS.contains(b[i] as char) {
                special += 1;
            }
        }

        if lower == 0 ||    // 2. 至少包含 一个小写英文 字母
            upper == 0 ||   // 3. 至少包含 一个大写英文 字母
            digit == 0 ||   // 4. 至少包含 一个数字
            special == 0 {  // 5. 至少包含 一个特殊字符 
            return false;
        }

        true
    }
}