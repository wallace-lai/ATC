struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let k = k as u32;
        for i in 0..nums.len() {
            if (i as u32).count_ones() == k {
                ans += nums[i];
            }
        }
        ans
    }
}