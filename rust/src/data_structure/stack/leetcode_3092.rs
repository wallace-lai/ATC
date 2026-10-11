struct Solution;

use std::collections::HashMap;
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_frequent_i_ds(nums: Vec<i32>, f: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut ans: Vec<i64> = vec![0; n];
        // num --> freq
        let mut freq: HashMap<i32, i64> = HashMap::new();
        // (freq, num)
        let mut heap: BinaryHeap<(i64, i32)> = BinaryHeap::new();

        for i in 0..n {
            let num = nums[i];
            let v = freq.entry(num).or_insert(0);
            *v += f[i] as i64;
            heap.push((*v, num));

            while let Some(top) = heap.peek() {
                if top.0 != freq[&top.1] {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some(top) = heap.peek() {
                ans[i] = top.0;
            }
        }

        ans
    }
}