struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn distance(v: &Vec<i32>) -> i32 {
        let mut min_dist = u32::MAX;
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                for k in (j + 1)..v.len() {
                    let ii = v[i];
                    let ji = v[j];
                    let ki = v[k];
                    let dist = ii.abs_diff(ji) + ji.abs_diff(ki) + ki.abs_diff(ii);
                    if dist < min_dist { min_dist = dist; }
                }
            }
        }

        min_dist as i32
    }

    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        // 数字 --> [下标]
        let mut count: HashMap<i32, Vec<i32>> = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.into_iter().enumerate() {
            count.entry(num).or_insert(vec![]).push(idx as i32);
        }

        let mut ans = i32::MAX;
        for (_, val) in count {
            if val.len() < 3 { continue; }
            // val.sort_unstable();
            let d = Self::distance(&val);
            if d < ans { ans = d; }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}