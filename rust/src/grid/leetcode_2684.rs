struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：DFS，性能差
    // 存在以下几个优化点
    // 1. 每次移动 dy = 1，列坐标严格递增，因此不可能回到已访问过的
    //      格子。vis数组是多余的
    // 2. 不同起点可能到达相同的中间格子 (i, j)，这些格子的后续最优
    //      路径会被反复计算，最坏情况下路径数可能更多，时间复杂度远
    //      高于 O(m * n)
    //
    // 由于移动方向只有向右（列j -> j+1），依赖关系天然是DAG。直接从
    // 右往左递推即可，无需递归，也无需vis数组
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {

        const DIR: [(i32, i32); 3] = [(-1, 1), (0, 1), (1, 1)];

        // 返回从 (x, y) 开始出发所能达到的最大格子数
        fn dfs(grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
            let m = grid.len();
            let n = grid[0].len();

            let mut area = 0;

            for (dx, dy) in DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                // 越界
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                // 已被访问过，或者所扩展格子里的值小于当前格子值
                let (nx, ny) = (nx as usize, ny as usize);
                if vis[nx][ny] || grid[nx][ny] <= grid[x][y] { continue; }

                vis[nx][ny] = true;
                area = area.max(dfs(grid, vis, nx, ny));
                vis[nx][ny] = false;
            }

            // 加上当前格子
            area + 1
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut vis = vec![vec![false; n]; m];

        let mut area = 0;
        for i in 0..m {
            vis[i][0] = true;
            area = area.max(dfs(&grid, &mut vis, i, 0));
            vis[i][0] = false;
        }

        area -= 1;
        area
    }

    #[cfg(any())]
    // 法二：动态规划（二维数组）
    // 定义 dp[i][j] 为从 (i, j) 出发，最多还能移动多少步
    // 转移：dp[i][j] = max(1 + dp[ni][j+1]), ni 属于 {i-1, i, i+1} 且 grid[ni][j+1] > grid[i][j]
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![0; n]; m];

        // 最优列dp[i][n - 1]默认为 0 （无法再移动）
        // 从倒数第二列开始，从右往左计算
        for j in (0..n - 1).rev() {
            for i in 0..m {
                let mut best = 0;

                let start = i.saturating_sub(1);
                let end = (i + 1).min(m - 1);

                for ni in start..=end {
                    // 值更大，意味着可以从 (i, j) 走到 (ni, j+1)
                    if grid[ni][j + 1] > grid[i][j] {
                        best = best.max(1 + dp[ni][j + 1]);
                    }
                }

                dp[i][j] = best;
            }
        }

        // 答案：从第 0 列任意一行出发的最大移动次数
        dp.iter().map(|row| row[0]).max().unwrap_or(0)
    }

    // 法三：动态规划（一维数组）
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // next[i] 表示从下一列第 i 行出发的最大移动次数
        // curr[i] 表示从当前列第 i 行出发的最大移动次数
        let mut next = vec![0; m];
        let mut curr = vec![0; m];

        // 从倒数第二列开始，从右往左进行计算
        for j in (0..n - 1).rev() {
            for i in 0..m {
                let mut best = 0;

                let start = i.saturating_sub(1);
                let end = (i + 1).min(m - 1);

                for ni in start..=end {
                    if grid[ni][j + 1] > grid[i][j] {
                        best = best.max(1 + next[ni]);
                    }
                }

                curr[i] = best;
            }

            std::mem::swap(&mut next, &mut curr);
        }

        // 注意，循环结束后 curr 和 next 进行了交换，所以从next数组中取
        *next.iter().max().unwrap()
    }
}