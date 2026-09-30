use std::i32;

struct Solution;

impl Solution {
    // O(n^2) - 0ms
    // pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    //     let mut ans = -1;
    //     for i in 0..nums.len() {
    //         for j in (i + 1)..nums.len() {
    //             let dif = nums[j] - nums[i];
    //             if dif > ans && dif > 0{
    //                 ans = dif;
    //             }
    //         }
    //     }

    //     ans
    // }

    // O(n) - 0ms
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        // 定义premin为nums[0..i - 1]中的最小值
        let mut ans = -1;
        let mut premin = nums[0];
        for i in 1..nums.len() {
            if nums[i] > premin {
                ans = ans.max(nums[i] - premin);
            } else {
                premin = nums[i];
            }
        }

        ans
    }
}