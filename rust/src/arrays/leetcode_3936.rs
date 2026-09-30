struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut left = 0 as i32;
        let mut right = nums.len() as i32 - 1;

        while left < right {
            while left < nums.len() as i32 &&
                nums[left as usize] != 0 {
                left += 1;
            }
            while right >= 0 &&
                nums[right as usize] == 0 {
                right -= 1;
            }

            if left < right {
                ans += 1;
                left += 1;
                right -= 1;
            }
        }

        ans
    }
}
