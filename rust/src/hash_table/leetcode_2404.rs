struct Solution;

use std::collections::HashMap;

impl Solution {
    // 性能差
    // pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    //     let mut map = HashMap::with_capacity(nums.len());
    //     for num in nums {
    //         *map.entry(num).or_insert(0) += 1;
    //     }

    //     let mut v: Vec<(i32, i32)> = map.into_iter()
    //         .map(|(key, val)| { (val, key) })
    //         .collect();
    //     v.sort_unstable_by(|a, b| {
    //         b.0.cmp(&a.0).then(
    //             a.1.cmp(&b.1)
    //         )
    //     });

    //     let mut ans = -1;
    //     for i in 0..v.len() {
    //         if v[i].1 & 1 == 0 {
    //             ans = v[i].1;
    //             break;
    //         }
    //     }

    //     ans
    // }

    // 性能优化
    // 1. 只统计偶数出现次数
    // 2. 使用min_by
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        // num --> cnt
        let mut map = HashMap::with_capacity(nums.len());
        for num in nums {
            if num & 1 == 0 {
                *map.entry(num).or_insert(0) += 1;
            }
        }
        if map.is_empty() { return -1; }

        // 先按次数（cnt）降序，再按数值（num）升序，取第一个（min）即可
        map.into_iter()
            .min_by(|a, b| {
                b.1.cmp(&a.1).then(a.0.cmp(&b.0))
            })
            .map(|(num, _)| num)
            .unwrap()
    }
}