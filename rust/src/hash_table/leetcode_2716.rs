struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut set = HashSet::with_capacity(32);
        for &c in s.as_bytes() {
            set.insert(c);
        }        

        set.len() as i32
    }
}