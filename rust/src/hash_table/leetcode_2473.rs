struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let v: Vec<i32> = map.into_iter()
            .map(|(_, val)| { val })
            .collect();
        if v.len() < 3 { return 0; }

        let mut ans = 0;
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                for k in (j + 1)..v.len() {
                    ans += v[i] * v[j] * v[k];
                }
            }
        }

        ans
    }
}