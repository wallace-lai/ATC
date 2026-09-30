struct Solution;

impl Solution {
    // 70ms，性能差
    // pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    //     use std::collections::HashSet;
    //     let n = nums.len() as i32;

    //     let mut v = HashSet::new();
    //     for j in 0..n {
    //         if nums[j as usize] != key { continue; }
    //         for d in 0..=k {
    //             if j - d >= 0 {
    //                 v.insert(j - d);
    //             }
    //         }
    //         for d in 1..=k {
    //             if j + d < n {
    //                 v.insert(j + d);
    //             }
    //         }
    //     }

    //     let mut ans: Vec<i32> = v.into_iter().collect();
    //     ans.sort_unstable();
    //     ans
    // }

    // 0ms
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut beg = None;
        let mut end = None;
        let n = nums.len() as i32;

        let mut v = Vec::new();
        for i in 0..n {
            if nums[i as usize] != key { continue; }

            // 初次查找到key
            if beg.is_none() {
                beg = Some(std::cmp::max(0, i - k));
                end = Some(std::cmp::min(n - 1, i + k));
            } else {
                beg = Some(std::cmp::max(end.unwrap() + 1, i - k));
                end = Some(std::cmp::min(n - 1, i + k));
            }
            for k in beg.unwrap()..=end.unwrap() {
                v.push(k);
            }
        }

        v
    }
}