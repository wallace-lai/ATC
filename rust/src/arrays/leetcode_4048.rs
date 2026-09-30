struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            m.entry(*n).or_insert(Vec::new()).push(i);
        }

        let mut ans = 0;
        for (_, v) in m.iter() {
            if v.len() == 3 &&
                v[1] - v[0] == v[2] - v[1] {
                ans += 1;
            }
        }
        ans
    }
}