struct Solution;

impl Solution {
    // 1ms，击败100%
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut ans = len as i32;

        let mut left = 0_i32;
        let mut right = len as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if target <= nums[mid as usize] {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        ans as i32
    }
}