struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let len = nums.len();
        let mut ans = i32::MIN;

        for i in 0..len {
            for j in i..len {
                if nums[j] <= 2 * nums[i] {
                    ans = ans.max(nums[i] ^ nums[j]);
                }
            }
        }

        ans
    }
}