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
            } else {
                r = m;
            }
        }

        nums[l as usize]
    }
}