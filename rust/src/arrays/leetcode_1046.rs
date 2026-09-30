struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    // 0ms，击败100%
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while heap.len() >= 2 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            if x != y {
                assert!(y > x);
                heap.push(y - x);
            }
        }

        if !heap.is_empty() { heap.pop().unwrap() } else { 0 }
    }
}