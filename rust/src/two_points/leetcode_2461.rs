struct Solution;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut ans = 0_i64;
        let mut sum = 0_i64;
        let mut left = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();

        for right in 0..nums.len() {
            sum += nums[right] as i64;
            *count.entry(nums[right]).or_insert(0) += 1;
            if right + 1 < k { continue; }

            // 此时窗口大小刚好为k
            if count.len() == k { ans = ans.max(sum); }

            if let Entry::Occupied(mut e) = count.entry(nums[left]) {
                sum -= nums[left] as i64;
                left += 1;

                let value = e.get_mut();
                *value -= 1;
                if *value == 0 { e.remove(); }
            }
        }

        ans
    }
}