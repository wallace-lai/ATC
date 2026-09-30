struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut ans = 0;
        for log in logs {
            match log.as_str() {
                "../" => { ans = std::cmp::max(0, ans - 1); },
                "./" => {},
                _ => { ans += 1; }
            }
        }

        ans
    }
}