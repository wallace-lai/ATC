struct Solution;

/// 子串为 [l..l + m]，其中 m 为子串的长度
fn segment_cost(s: &[u8], l: usize, m: usize, factors: &Vec<Vec<usize>>) -> i32 {
    let mut best = m as i32;
    for &d in &factors[m] {
        let g = m / d;
        let mut changes = 0;
        for offset in 0..d {
            for t in 0..g / 2 {
                // 组内首尾配对
                if s[l + offset + t * d] !=
                    s[l + offset + (g - 1 - t) * d] {
                    changes += 1;
                }
            }
        }
        best = best.min(changes);
    }

    best
}

fn solve(seg: usize, start: usize, n: usize, cost: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if seg == 1 { return cost[start][n - 1]; }
    if memo[seg][start] != -1 { return memo[seg][start]; }

    let mut best = n as i32;
    let limit = n - 1 - 2 * (seg - 1);
    for e in (start + 1)..=limit {
        best = best.min(
            cost[start][e] +
            solve(seg - 1, e + 1, n, cost, memo)
        );
    }

    memo[seg][start] = best;
    best
}

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let k = k as usize;
        let b = s.as_bytes();
        let n = b.len();

        // 预处理每个数的真因子
        // factors[i] 存储 i 的所有真因子
        let mut factors = vec![vec![]; n + 1];
        for d in 1..=n / 2 {
            let mut m = d * 2;
            while m <= n {
                factors[m].push(d);
                m += d;
            }
        }

        // 计算每段的修改次数
        // cost[l][r] 表示子串 [l, r] 改成回文串的修改次数
        let mut cost = vec![vec![0; n]; n];
        for l in 0..n - 1 {
            for r in (l + 1)..n {
                // 子串长度至少为 2
                cost[l][r] = segment_cost(b, l, r - l + 1, &factors);
            }
        }

        let mut memo = vec![vec![-1_i32; n]; k + 1];
        solve(k, 0, n, &cost, &mut memo)
    }
}