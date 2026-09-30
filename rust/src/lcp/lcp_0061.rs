struct Solution;

use std::cmp::Ordering;

impl Solution {
    // 2ms，击败100%
    pub fn temperature_trend(temp_a: Vec<i32>, temp_b: Vec<i32>) -> i32 {
        assert!(temp_a.len() >= 2);
        assert!(temp_a.len() == temp_b.len());

        let len = temp_a.len() - 1;
        let mut trend_cmp = vec![0; len];
        for i in 0..len {
            let ta = match temp_a[i + 1].cmp(&temp_a[i]) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => -1,
            };
            let tb = match temp_b[i + 1].cmp(&temp_b[i]) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => -1,
            };

            // 两边趋势相同赋值为0，趋势不同赋值为1
            trend_cmp[i] = if ta == tb { 0 } else { 1 };
        }

        // println!("cmp is {:?}", trend_cmp);

        let mut max_len = 0;
        let mut cur_len = 0;
        for x in trend_cmp {
            if x == 0 {
                cur_len += 1;
                if cur_len > max_len { max_len = cur_len; }
            } else {
                cur_len = 0
            }
        }

        max_len
    }
}