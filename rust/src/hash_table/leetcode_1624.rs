struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut m: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let val = m.entry(c).or_insert(vec![]);
            val.push(i);
        }

        let mut ans = -1;
        for (_, val) in m {
            if val.len() > 1 {
                let len = val[val.len() - 1] - val[0] - 1;
                ans = ans.max(len as i32);
            }
        }

        ans
    }
}