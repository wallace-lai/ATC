struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let s: HashSet<i32> = arr.iter().map(|i| *i).collect();
        let mut num = 1;
    
        loop {
            if !s.contains(&num) {
                k -= 1;
                if k == 0 { break; }
            }
            num += 1;
        }

        num
    }
}