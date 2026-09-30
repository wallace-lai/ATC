struct Solution;

impl Solution {
    // 法一：2ms，击败100%
    // pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    //     let mut sum = nums.into_iter().sum();
    //     if sum % k == 0 { return 0; }
    //     while sum > k { sum -= k; }
    //     sum
    // }

    // 法二：0ms，击败100%
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum: i32 = nums.into_iter().sum();
        sum % k
    }
}