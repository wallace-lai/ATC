struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        fn _do(sum: &mut Vec<i64>, todo: &mut Vec<i32>, o: i32, l: i32, r: i32, add: i32) {
            let o = o as usize;
            sum[o] += add as i64 * (r - l + 1) as i64;
            todo[o] += add;
        }

        fn query_and_add_1(sum: &mut Vec<i64>, todo: &mut Vec<i32>, o: i32, l: i32, r: i32, L: i32, R: i32)-> i64 {
            if L <= l && r <= R {
                let res: i64 = sum[o as usize];
                _do(sum, todo, o, l, r, 1);
                return res;
            }

            let m = (l + r) / 2;
            let add = todo[o as usize];
            if add != 0 {
                _do(sum, todo, o * 2, l, m, add);
                _do(sum, todo, o * 2 + 1, m + 1, r, add);
                todo[o as usize] = 0;
            }

            let mut res: i64 = 0;
            if L <= m { res += query_and_add_1(sum, todo, o * 2, l, m, L, R); }
            if m < R { res += query_and_add_1(sum, todo, o * 2 + 1, m + 1, r, L, R); }
            sum[o as usize] = sum[o as usize * 2] + sum[o as usize * 2 + 1];
            res
        }

        let n = nums.len();
        let mut todo: Vec<i32> = vec![0; 4 * n];
        let mut sum: Vec<i64> = vec![0; 4 * n];

        let mut s: i64 = 0;
        let mut ans: i64 = 0;
        let mut last: HashMap<i32, i32> = HashMap::new();

        for i in 1..=n {
            let x = nums[i - 1];
            let j = if last.contains_key(&x) { last[&x] } else { 0 };
            s += query_and_add_1(&mut sum, &mut todo, 1, 1, n as i32, j + 1, i as i32) * 2 + i as i64 - j as i64;
            ans = (ans + s) % 1_000_000_007;
            last.insert(x, i as i32);
        }

        ans as i32
    }
}