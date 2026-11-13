struct Solution;

impl Solution {
    // 法一：二分搜索
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l < r {
            let m = l + (r - l) / 2;
            let (um, ur) = (m as usize, r as usize);
            if nums[um] > nums[ur] {
                l = m + 1;
            } else if nums[um] < nums[ur] {
                r = m;
            } else {
                // nums[um] == nums[ur]
                // 中间的数字等于右边的数字，则最小值可能为该
                // 重复数字，或者最小值在该重复数字的左侧
                r -= 1;
            }
        }

        nums[l as usize]
    }
}