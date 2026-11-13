struct Solution;

impl Solution {
    // 法一：二分搜索
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            // 处理重复数字
            // 左边有重复数字，将左边界右移
            // 右边有重复数字，将右边界左移
            while l < r && nums[l as usize] == nums[l as usize + 1] {
                l += 1;
            }
            while l < r && nums[r as usize] == nums[r as usize - 1] {
                r -= 1;
            }

            let m = l + (r - l) / 2;
            let (ul, um, ur) = (l as usize, m as usize, r as usize);

            if nums[um] == target {
                return true;
            }

            // 注意：判断左半段是否有序要用 <=，而不是 <
            if nums[ul] <= nums[um] {
                // 如果 nums[l..=m] 是有序的
                if nums[ul] <= target && target < nums[um] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                // 如果 nums[m..=r] 是有序的
                if nums[um] < target && target <= nums[ur] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }

        false
    }
}