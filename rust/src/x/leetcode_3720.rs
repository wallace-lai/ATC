struct Solution;

impl Solution {
    pub fn get_max_string(count: &[i32]) -> String {
        let sum = count.iter().map(|c| *c as usize).sum();
        let mut ans = String::with_capacity(sum);
        for i in (0..count.len()).rev() {
            let repeat = count[i] as usize;
            let to_add = (i as u8 + b'a') as char;
            ans.push_str(&to_add.to_string().repeat(repeat));
        }

        ans
    }

    pub fn get_min_string(count: &[i32]) -> String {
        let sum = count.iter().map(|c| *c as usize).sum();
        let mut ans = String::with_capacity(sum);
        for i in 0..count.len() {
            let repeat = count[i] as usize;
            let to_add = (i as u8 + b'a') as char;
            ans.push_str(&to_add.to_string().repeat(repeat));
        }

        ans
    }

    pub fn can_form_greater(count: &[i32], target: &str) -> bool {
        let max_str = Self::get_max_string(count);
        max_str.as_str() > target
    }

    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            count[idx] += 1;
        }

        let mut ans = String::new();
        for (i, c) in target.chars().enumerate() {
            let target_char = (c as u8 - b'a') as usize;

            // 情况一：先尝试在当前位置放置与target[i]相同的字符
            if count[target_char] > 0 {
                count[target_char] -= 1;
                // 检查剩余字符串能否构成大于target[i + 1..]的字符串
                if Self::can_form_greater(&count, &target[i + 1..]) {
                    ans.push(c);
                    continue;
                }
                // 不能构成更大的字符串，回溯
                count[target_char] += 1;
            }

            // 情况二：在当前位置放置一个大于target[i]的字符
            for j in (target_char + 1)..count.len() {
                if count[j] > 0 {
                    count[j] -= 1;
                    ans.push((b'a' + j as u8) as char);
                    // 剩余位置按照最小字典序填充即可
                    ans.push_str(&Self::get_min_string(&count));
                    return ans;
                }
            }

            return String::new();
        }

        String::new()
    }
}