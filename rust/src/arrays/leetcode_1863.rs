struct Solution;

impl Solution {
    // 2ms
    // pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    //     let n = nums.len();
    //     let mut ans = 0;

    //     for mask in 0..(1_usize << n) {
    //         let mut xor = 0;
    //         for i in 0..n {
    //             if mask & (1_usize << i) != 0 {
    //                 xor ^= nums[i];
    //             }
    //         }
    //         ans += xor;
    //     }

    //     ans
    // }

    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &Vec<i32>, idx: usize, sum: i32) -> i32 {
            if idx == nums.len() {
                return sum;
            }
            dfs(nums, idx + 1, sum) +
            dfs(nums, idx + 1, sum ^ nums[idx])
        }
        dfs(&nums, 0, 0)
    }
}