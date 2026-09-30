struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut ans = 0;
        let mut map = HashMap::new();
        for i in low_limit..=high_limit {
            let sum = {
                let mut sum = 0;
                let mut n = i;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                }
                sum
            };
            let val = map.entry(sum).or_insert(0);
            ans = ans.max(*val + 1);
            *val += 1;
        }

        ans
    }
}