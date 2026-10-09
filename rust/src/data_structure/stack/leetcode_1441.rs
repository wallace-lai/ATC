struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut idx = 0;
        let mut ans = Vec::with_capacity(target.len());

        for i in 1..=n {
            if i == target[idx] {
                ans.push("Push".to_string());
                idx += 1;
            } else if i < target[idx] {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
            }

            if idx >= target.len() { break; }
        }

        ans
    }
}
