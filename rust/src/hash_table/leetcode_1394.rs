struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut m = HashMap::with_capacity(arr.len());
        for num in arr {
            *m.entry(num).or_insert(0) += 1;
        }

        let mut ans = -1;
        for (num, freq) in m {
            if num == freq && num > ans {
                ans = num;
            }
        }

        ans
    }
}