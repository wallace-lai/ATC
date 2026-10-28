struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：DFS（超时）
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        
        // 从 (x, y) 出发到达右下角并记录异或值
        // 由于每一步只能向右或者向下，因此无需 visited 数组
        fn dfs(g: &Vec<Vec<i32>>, x: usize, y: usize, v: i32, ans: &mut i32) {
            let m = g.len();
            let n = g[0].len();
            if x == m - 1 && y == n - 1 {
                *ans = (v ^ g[x][y]).min(*ans);
                return;
            }

            // 向下
            if x < m - 1 {
                dfs(g, x + 1, y, v ^ g[x][y], ans);
            }
            // 向右
            if y < n - 1 {
                dfs(g, x, y + 1, v ^ g[x][y], ans);
            }
        }

        let mut ans = i32::MAX;
        dfs(&grid, 0, 0, 0, &mut ans);
        ans
    }

    #[cfg(any())]
    // 法二：动态规划（dp数组空间未优化，三维dp）
    pub fn min_cost(g: Vec<Vec<i32>>) -> i32 {
        let m = g.len();
        let n = g[0].len();
        const XOR_RANGE: usize = 1024;

        // dp[i][j][k] = 到达 (i, j) 时，是否可能得到异或值 k
        let mut dp = vec![
            vec![
                vec![false; XOR_RANGE];
                n
            ];
            m
        ];

        // 起点
        dp[0][0][g[0][0] as usize] = true;

        // 第一行：只能从左边过来
        for j in 1..n {
            let curr = g[0][j] as usize;
            for k in 0..XOR_RANGE {
                if dp[0][j - 1][k] {
                    dp[0][j][k ^ curr] = true;
                }
            }
        }

        // 第一列：只能从上边过来
        for i in 1..m {
            let curr = g[i][0] as usize;
            for k in 0..XOR_RANGE {
                if dp[i - 1][0][k] {
                    dp[i][0][k ^ curr] = true;
                }
            }
        }

        // 其余格子：可从上边或者左边过来
        for i in 1..m {
            for j in 1..n {
                let curr = g[i][j] as usize;
                for k in 0..XOR_RANGE {
                    if dp[i - 1][j][k] || dp[i][j - 1][k] {
                        dp[i][j][k ^ curr] = true;
                    }
                }
            }
        }

        // 在终点的所有可能异或值中找最小值
        for k in 0..XOR_RANGE {
            if dp[m - 1][n - 1][k] {
                return k as i32;
            }
        }

        unreachable!();
    }

    // 法三：动态规划（二维dp）
    // dp[i][j]只依赖当前行和上一行的状态，可以使用滚动数组将空间复杂度
    // 从 O(m*n*1024) 优化到 (n*1024)
    pub fn min_cost(g: Vec<Vec<i32>>) -> i32 {
        let m = g.len();
        let n = g[0].len();
        const XOR_RANGE: usize = 1024;

        // dp[j]表示当前行第 j 列的所有可能的异或值
        let mut dp = vec![vec![false; XOR_RANGE]; n];

        // 起点
        dp[0][g[0][0] as usize] = true;

        // 第一行：只能从左边过来
        for j in 1..n {
            let curr = g[0][j] as usize;
            for k in 0..XOR_RANGE {
                if dp[j - 1][k] {
                    dp[j][k ^ curr] = true;
                }
            }
        }

        // 后续的行
        for i in 1..m {
            // 当前行的第一列：只能从上方过来
            // 此时 dp[0] 还保存着上一行的状态，所以可以直接基于它更新
            let mut new_dp0 = vec![false; XOR_RANGE];
            let curr = g[i][0] as usize;
            for k in 0..XOR_RANGE {
                if dp[0][k] {
                    new_dp0[k ^ curr] = true;
                }
            }
            dp[0] = new_dp0;

            // 当前行的其他列
            for j in 1..n {
                let mut new_dpj = vec![false; XOR_RANGE];
                let curr = g[i][j] as usize;
                for k in 0..XOR_RANGE {
                    // 来自上方（dp[j]尚未更新，仍是上一行的状态）
                    if dp[j][k] {
                        new_dpj[k ^ curr] = true;
                    }
                    // 来自左方（dp[j - 1]已被本行更新）
                    if dp[j - 1][k] {
                        new_dpj[k ^ curr] = true;
                    }
                }
                dp[j] = new_dpj;
            }
        }

        // 在终点格子的所有可能异或值中找最小值
        for k in 0..XOR_RANGE {
            if dp[n - 1][k] {
                return k as i32;
            }
        }

        unreachable!();
    }
}
