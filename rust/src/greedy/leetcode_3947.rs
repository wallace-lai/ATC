struct Solution;

use std::collections::HashMap;

impl Solution {
    // 法一：贪心
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, mut budget: i32) -> i32 {
        let n = items.len();
        let mut cnt_factor = vec![0; n + 1];
        let mut min_price = i32::MAX;
        for p in items.iter() {
            let factor = p[0] as usize;
            let price = p[1];
            cnt_factor[factor] += 1;
            min_price = min_price.min(price);
        }

        let mut cnt_multi = vec![0; n + 1];
        let mut sum_cnt: HashMap<i32, i64> = HashMap::new();
        for p in items {
            let factor = p[0] as usize;
            let price = p[1];
            if price >= min_price * 2 {
                continue;
            }

            let cnt = &mut cnt_multi[factor];
            if *cnt == 0 {
                for j in (factor..=n).step_by(factor) {
                    *cnt += cnt_factor[j];
                } 
            }
            if *cnt > 1 {
                *sum_cnt.entry(price).or_insert(0) += *cnt - 1;
            }
        }

        let mut entries: Vec<(i32, i64)> = sum_cnt.into_iter().collect();
        entries.sort_unstable_by_key(|&(price, _)| price);

        let mut ans = 0;
        for (price, cnt) in entries {
            if budget < price { break; }
            let c = cnt.min(1i64 * budget as i64 / price as i64);
            budget -= price * c as i32;
            ans += c * 2;
        }

        ans as i32 + budget / min_price
    }
}