struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut bytes = s.as_bytes().to_vec();
        bytes.reverse();

        let mut set: HashSet<&[u8]> = HashSet::with_capacity(s.len());
        for slice in bytes.windows(2) {
            set.insert(slice);
        }

        for slice in s.as_bytes().windows(2) {
            if set.contains(slice) { return true; }
        }

        false
    }
}