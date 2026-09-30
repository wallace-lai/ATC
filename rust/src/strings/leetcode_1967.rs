struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;
        for i in 0..patterns.len() {
            if word.contains(patterns[i].as_str()) {
                ans += 1;
            }
        }

        ans
    }
}