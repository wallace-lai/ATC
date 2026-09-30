use std::mem::swap;

struct Solution;

impl Solution {
    // O(n^2)
    // pub fn max_product(mut nums: Vec<i32>) -> i32 {
    //     nums.sort_unstable();
    //     let a = nums[nums.len() - 1] - 1;
    //     let b = nums[nums.len() - 2] - 1;
    //     a * b
    // }

    // O(n)
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max1 = nums[0];
        let mut max2 = nums[1];
        if max2 > max1 {
            swap(&mut max1, &mut max2);
        }

        for i in 2..nums.len() {
            if nums[i] > max1 {
                max2 = max1;
                max1 = nums[i];
            } else if nums[i] > max2 {
                max2 = nums[i];
            }
        }

        (max1 - 1) * (max2 - 1)
    }
}