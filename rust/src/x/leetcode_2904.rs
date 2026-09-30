struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let count = bytes.iter()
            .filter(|b| **b == b'1')
            .count();
        if count < k as usize { return "".to_string(); }

        let mut ans = s.clone();
        let mut ones = 0;
        let mut left = 0;

        for right in 0..bytes.len() {
            if bytes[right] == b'1' {
                ones += 1;
            }

            while ones > k || bytes[left] == b'0' {
                if bytes[left] == b'1' {
                    ones -= 1;
                }
                left += 1;
            }

            if ones == k {
                let tmp = &s[left..=right];
                if tmp.len() < ans.len() ||
                    (tmp.len() == ans.len() && tmp < ans.as_str()) {
                    ans = tmp.to_string();
                }
            }
        }

        ans
    }
}