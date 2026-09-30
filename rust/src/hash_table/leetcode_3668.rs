struct Solution;

use std::collections::HashSet;

impl Solution {
    // 0ms，击败100%
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(friends.len());
        let set: HashSet<i32> = friends.into_iter().collect();

        for o in order {
            if set.contains(&o) {
                ans.push(o);
            }
        }

        ans
    }
}