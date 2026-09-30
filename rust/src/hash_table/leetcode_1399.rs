struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut m = HashMap::with_capacity(n as usize);
        for i in 1..=n {
            let sum = {
                let mut sum = 0;
                let mut tmp = i;
                while tmp > 0 {
                    sum += tmp % 10;
                    tmp /= 10;
                }
                sum
            };

            m.entry(sum).or_insert(vec![]).push(i);
        }

        let max_len = m.iter()
            .map(|(_, i)| i.len())
            .max()
            .unwrap();

        let ans = m.iter()
            .filter(|(_, i)| i.len() == max_len)
            .fold(0, |acc, _| acc + 1);

        ans
    }
}