
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &value) in nums.iter().enumerate() {
            let complement = target - value;
            if let Some(&j) = map.get(&complement) {
                if j != i {
                    return vec![j as i32, i as i32];
                }
            }
            map.insert(value, i);
        }

        vec![-1, -1]
    }
}