struct Solution;

use std::collections::HashMap;

impl Solution {
    // 0ms，击败100%
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for &num in nums.iter() {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut ans = -1;
        for i in 0..nums.len() {
            let key = nums[i];
            if let Some(&val) = count.get(&key) {
                if key & 1 == 0 && val == 1 {
                    ans = key;
                    break;
                }
            }
        }

        ans
    }
}