struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_pos = 0;
        let mut max_pos = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num > nums[max_pos] { max_pos = i; }
            if num < nums[min_pos] { min_pos = i; }
        }

        let rmv_min_cnt = (min_pos + 1).min(n - min_pos);
        let rmv_max_cnt = (max_pos + 1).min(n - max_pos);
        let mut ans = 0;
        if rmv_min_cnt < rmv_max_cnt {
            // 移除的是最小值
            ans += rmv_min_cnt;
            if min_pos < max_pos {
                // 左侧的最小值被移除，最大值在右侧
                ans += (max_pos - min_pos).min(n - max_pos);
            } else {
                // 右侧的最小值被移除，最大值在左侧
                ans += (max_pos + 1).min(min_pos - max_pos);
            }
        } else {
            // 移除的是最大值
            ans += rmv_max_cnt;
            if min_pos < max_pos {
                // 右侧的最大值被移除，最小值在左侧
                ans += (min_pos + 1).min(max_pos - min_pos);
            } else {
                // 左侧的最大值被移除，最小值在右侧
                ans += (min_pos - max_pos).min(n - min_pos);
            }
        }

        ans as i32
    }
}