use std::collections::HashMap;

struct Solution;

impl Solution {
    // O(n^4) - 6ms
    // pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    //     let mut ans = 0;
    //     for a in 0..nums.len() {
    //         for b in (a + 1)..nums.len() {
    //             for c in (b + 1)..nums.len() {
    //                 for d in (c + 1)..nums.len() {
    //                     if nums[a] + nums[b] + nums[c] == nums[d] {
    //                         ans += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     ans
    // }

    // O(n^3) - 7ms
    // pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    //     let len = nums.len();
    //     let mut ans = 0;
    //     let mut m: HashMap<i32, i32> = HashMap::with_capacity(len);

    //     for c in (2..(len - 1)).rev() {
    //         *m.entry(nums[c + 1]).or_insert(0) += 1;
    //         for a in 0..c {
    //             for b in (a + 1)..c {
    //                 let sum = nums[a] + nums[b] + nums[c];
    //                 if let Some(val) = m.get(&sum) {
    //                     ans += *val;
    //                 }
    //             }
    //         }
    //     }

    //     ans
    // }

    // O(n^2)
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut ans = 0;
        let mut m: HashMap<i32, i32> = HashMap::with_capacity(len);

        for b in (1..(len - 2)).rev() {
            for d in (b + 2)..len {
                *m.entry(nums[d] - nums[b + 1]).or_insert(0) += 1;
            }
            for a in 0..b {
                let sum = nums[a] + nums[b];
                if let Some(val) = m.get(&sum) {
                    ans += *val;
                }
            }
        }

        ans
    }
}