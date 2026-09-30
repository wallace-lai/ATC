struct Solution;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    // 3ms，击败100%
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        let mut sum;
        let mut map = HashMap::with_capacity(nums.len());
        let mut heap = BinaryHeap::with_capacity(nums.len());

        for window in nums.windows(k as usize) {
            sum = 0;
            map.clear();
            heap.clear();

            for &num in window.iter() {
                sum += num;
                *map.entry(num).or_insert(0) += 1;
            }

            if map.len() < x as usize {
                ans.push(sum);
                continue;
            }
            
            heap = map.iter()
                .map(|(key, val)| (*val, *key))
                .collect();

            sum = 0;
            for _ in 0..x {
                if let Some((val, key)) = heap.pop() {
                    sum += key * val;
                }
            }
            ans.push(sum);
        }

        ans
    }
}