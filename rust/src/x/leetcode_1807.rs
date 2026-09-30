struct Solution;

use std::collections::HashMap;

impl Solution {
    // 27ms
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let m: HashMap<&str, &str> = knowledge.iter()
            .map(|k| (k[0].as_str(), k[1].as_str()))
            .collect();

        let n = s.len();
        let b = s.as_bytes();
        let mut ans = String::with_capacity(n);
        let mut start = None;
        
        for i in 0..n {
            if start.is_none() {
                if b[i] == b'(' {
                    start = Some(i);
                } else {
                    ans.push(b[i] as char);
                }
            } else {
                if b[i] == b')' {
                    let key = &s[start.unwrap() + 1..i];
                    if let Some(&val) = m.get(key) {
                        ans.push_str(val);
                    } else {
                        ans.push('?');
                    }
                    start = None;
                }
            }
        }

        ans
    }
}