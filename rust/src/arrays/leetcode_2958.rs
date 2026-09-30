struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut ans = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            // 窗口往右扩展一位，记录扩展后的计数
            let push = nums[right];
            let push_count = {
                let entry = m.entry(push).or_insert(0);
                *entry += 1;
                *entry
            };
            let mut push_count = push_count;

            // 若计数超过了k，则窗口需要收缩
            while push_count > k {
                let pop = nums[left];
                // pop必定存在于哈希表中，直接使用get_mut
                if let Some(count) = m.get_mut(&pop) {
                    *count -= 1;
                    if pop == push { push_count -= 1; }
                }
                left += 1;
            }

            // 更新答案
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}