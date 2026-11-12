struct Solution;

impl Solution {
    // 法一：二分搜索
    // 题目限定：数组中没有重复数字
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let (ul, um, ur) = (l as usize, m as usize, r as usize);

            if nums[um] == target {
                return m;
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

        -1
    }

    // 法二：找最小值点，然后做两段 lower_bound
}
