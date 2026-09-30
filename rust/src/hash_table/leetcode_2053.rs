struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut m = HashMap::with_capacity(arr.len());
        for s in &arr {
            *m.entry(s.as_str()).or_insert(0) += 1;
        }

        let mut idx = 0;
        for s in &arr {
            if let Some(val) = m.get(s.as_str()) {
                if *val == 1 {
                    idx += 1;
                    if idx == k {
                        return s.clone();
                    }
                }
            }
        }

        "".to_string()
    }
}