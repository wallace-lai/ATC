struct Solution;

use std::collections::HashSet;

impl Solution {
    // WA
    // pub fn missing_integer(nums: Vec<i32>) -> i32 {
    //     let mut max_len = 1;
    //     let mut max_sum = nums[0];
    //     let mut pre = nums[0];
    //     let mut len = 1;
    //     let mut sum = nums[0];
    //     let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
    //     set.insert(nums[0]);

    //     for i in 1..nums.len() {
    //         set.insert(nums[i]);

    //         if nums[i] == pre + 1 {
    //             pre = nums[i];
    //             len += 1;
    //             sum += nums[i];

    //             if len > max_len { max_len = len; }
    //             if sum > max_sum { max_sum = sum; }
    //         } else {
    //             pre = nums[i];
    //             len = 1;
    //             sum = nums[i];
    //         }
    //     }
    //     // println!("max_len is {max_len}, max_sum is {max_sum}");

    //     while set.contains(&max_sum) {
    //         max_sum += 1;
    //     }

    //     max_sum
    // }

    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().copied().collect();

        let mut end = 0;
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                end += 1;
                sum += nums[i];
            } else {
                break;
            }
        }
        println!("end is {end}, sum is {sum}");

        while set.contains(&sum) {
            sum += 1;
        }

        sum
    }
}