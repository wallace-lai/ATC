struct Solution;

impl Solution {
    pub fn count_eval(s: String, result: i32) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();

        // memo[i][j] = Some((true_count, false_count)), i <= j
        let mut memo: Vec<Vec<Option<(i64, i64)>>> = vec![vec![None; n]; n];

        // 返回区间 [i, j] 得到 true 和 false 的方案数
        fn dfs(
            s: &[char],
            i: usize,
            j: usize,
            memo: &mut Vec<Vec<Option<(i64, i64)>>>
        ) -> (i64, i64) {
            // 已经计算过了
            if let Some(v) = memo[i][j] { return v; }

            let (mut t, mut f) = (0, 0);

            if i == j {
                if s[i] == '1' { t = 1; } else { f = 1; }
            } else {
                // 只在奇数位置枚举运算符
                let mut k = i + 1;
                while k < j {
                    let (lt, lf) = dfs(s, i, k - 1, memo);
                    let (rt, rf) = dfs(s, k + 1, j, memo);
                    
                    match s[k] {
                        '&' => {
                            t += lt * rt;
                            f += lf * rt + lt * rf + lf * rf;
                        },
                        '|' => {
                            t += lt * rt + lt * rf + lf * rt;
                            f += lf * rf;
                        },
                        '^' => {
                            t += lt * rf + lf * rt;
                            f += lt * rt + lf * rf;
                        },
                        _ => {}
                    }

                    k += 2;
                }
            }

            memo[i][j] = Some((t, f));
            (t, f)
        }

        let (t, f) = dfs(&s, 0, n - 1, &mut memo);
        if result == 1 { t as i32 } else { f as i32 }
    }
}