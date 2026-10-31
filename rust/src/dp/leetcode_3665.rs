struct Solution;

impl Solution {
    #[cfg(any())]
    /// 法一：记忆化搜索，超时
    /// 定义 dfs(i, j, k) 表示从方向 k 来到 (i, j) 的情况下，从 (i, j) 倒着
    /// 回到起点 (0, 0) 的方案数
    /// k = 0 表示从右边来到 (i, j)
    /// k = 1 表示从下边来到 (i, j)
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();
        let mut memo: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1, -1]; n]; m];

        fn dfs(grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Vec<i32>>>, i: i32, j: i32, k: usize) -> i32 {
            // 越界
            if i < 0 || j < 0 { return 0; }
            // 到达起点
            if i == 0 && j == 0 { return 1; }
            // 之前计算过
            let (ui, uj) = (i as usize, j as usize);
            if memo[ui][uj][k] != -1 { return memo[ui][uj][k]; }

            let mut ans = 0;
            if grid[ui][uj] == 0 {
                // 没有镜子，随便走
                ans = (dfs(grid, memo, i, j - 1, 0) + dfs(grid, memo, i - 1, j, 1)) % MOD;
            } else if k == 0 {
                ans = dfs(grid, memo, i - 1, j, 1);
            } else {
                ans = dfs(grid, memo, i, j - 1, 0);
            }

            ans
        }

        dfs(&grid, &mut memo, m as i32 - 1, n as i32 - 1, 0)
    }

    /// 法二：三维 DP
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![vec![[0, 0]; n+1]; m+1];

        f[0][1] = [1, 1];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    f[i + 1][j + 1][0] = (f[i + 1][j][0] + f[i][j + 1][1]) % MOD;
                    f[i + 1][j + 1][1] = f[i + 1][j + 1][0];
                } else {
                    f[i + 1][j + 1][0] = f[i][j + 1][1];
                    f[i + 1][j + 1][1] = f[i + 1][j][0];
                }
            }
        }

        f[m][n][0]
    }
}