struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut m = HashMap::with_capacity(s.len());
        for &c in s.as_bytes() {
            *m.entry(c).or_insert(0) += 1;
        }

        let num;
        match m.iter().next() {
            Some((_, val)) => { num = *val; }
            None => { return false; }
        }

        for (_, val) in m.iter() {
            if num != *val {
                return false
            }
        }

        true
    }
}