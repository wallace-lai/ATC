struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for i in 0..nums.len() {
            let x = nums[i];
            if i as i32 != x {
                ans &= x;
            }
        }

        std::cmp::max(ans, 0)
    }
}