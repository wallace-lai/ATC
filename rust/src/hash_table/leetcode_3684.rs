struct Solution;

use std::collections::HashSet;
use std::collections::BinaryHeap;

impl Solution {
    // 0ms，击败100%
    pub fn max_k_distinct(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut heap: BinaryHeap<i32> = set.into_iter().collect();
        let mut ans = vec![];

        while k > 0 && !heap.is_empty() {
            ans.push(heap.pop().unwrap());
            k -= 1;
        }

        ans
    }
}