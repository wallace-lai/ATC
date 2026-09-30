struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut set = HashSet::new();
        for range in ranges {
            for i in range[0]..=range[1] {
                set.insert(i);
            }
        }

        for i in left..=right {
            if !set.contains(&i) {
                return false;
            }
        }
        true
    }
}