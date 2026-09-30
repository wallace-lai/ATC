struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut count = HashMap::with_capacity(len);
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        // (num, freq)
        let mut v: Vec<(i32, i32)> = count.into_iter().collect();
        v.sort_unstable_by(|a, b| {
            a.1.cmp(&b.1).then(b.0.cmp(&a.0))
        });
        
        let mut ans = Vec::with_capacity(len);
        for (num, freq) in v.into_iter() {
            let slice = vec![num; freq as usize];
            ans.extend(slice);
        }

        ans
    }
}