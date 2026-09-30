struct Solution;

use std::collections::HashMap;

impl Solution {
    // O(n^2) - 0ms
    // pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    //     let len = nums.len();
    //     let mut ans = 0;

    //     for i in (0..(len - 1)) {
    //         for j in ((i + 1)..len) {
    //             let d = nums[i] - nums[j];
    //             if d == k || d == -k {
    //                 ans += 1;
    //             }
    //         }
    //     }

    //     ans
    // }

    // O(n) - 3ms，运行时间反而更长了
    // pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    //     let len = nums.len();
    //     let mut ans = 0;
    //     let mut m: HashMap<i32, i32> = HashMap::with_capacity(len);

    //     // i从倒数第二个数开始往前遍历
    //     for i in (0..(len - 1)).rev() {
    //         // i每往前遍历一个数，j多了一个可选数，即j = i+1。每多一个j，
    //         // 则新增两个符合条件的数，即k + nums[j]和-k + nums[j]
    //         let t1 = nums[i + 1] + k;
    //         let t2 = nums[i + 1] - k;
    //         *m.entry(t1).or_insert(0) += 1;
    //         *m.entry(t2).or_insert(0) += 1;

    //         if let Some(val) = m.get(&nums[i]) {
    //             ans += *val;
    //         }
    //     }

    //     ans
    // }

    // O(n) - 0ms
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut ans = 0;
        let mut m: HashMap<i32, i32> = HashMap::with_capacity(len);

        for j in 0..len {
            // 枚举j，若之前遇到了 nums[j]-k 或者 nums[j]+k
            // 意味着遇到了符合的(i, j)对
            let t1 = nums[j] + k;
            let t2 = nums[j] - k;
            if let Some(val) = m.get(&t1) {
                ans += *val;
            }
            if let Some(val) = m.get(&t2) {
                ans += *val;
            }

            *m.entry(nums[j]).or_insert(0) += 1;
        }

        ans
    }
}