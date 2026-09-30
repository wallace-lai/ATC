struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap: BinaryHeap<i32> = gifts.iter().map(|g| *g).collect();

        for _ in 0..k {
            if let Some(mut v) = heap.pop() {
                v = v.isqrt();
                heap.push(v);
            }
        }

        heap.iter().map(|g| *g as i64).sum()
    }
}