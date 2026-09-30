use std::i32;

struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let mut ans = -1;
        let mut cap_min = i32::MAX;
        for (i, cap) in capacity.into_iter().enumerate() {
            if cap >= item_size && cap < cap_min {
                cap_min = cap;
                ans = i as i32;
            }
        }

        ans
    }
}