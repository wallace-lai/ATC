struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：DFS
    // 存在以下几个可优化的点：
    // 1. 递归 DFS 有栈溢出风险，改用显式栈
    // 2. 每次递归都计算 m 和 n
    // 3. 对于已经触碰边界的连通分量，没有必要继续累加面积并向上传播
    // 4. 使用 多源显式栈 + 边界淹没法 性能更优
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        
        // 如果地块全部被 0 包围，则说明地块中的单元格无法离开网格边界
        // 0 - 水域
        // 1 - 陆地
        // 从 (x, y) 开始标记陆地，返回地块面积以及是否封闭
        fn dfs(g: &mut Vec<Vec<i32>>, x: i32, y: i32) -> (i32, bool) {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界：说明岛屿触碰了边界，不是封闭的
            if x < 0 || x >= m || y < 0 || y >= n {
                return (0, false);
            }

            // 遇到水域，停止搜索
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] == 0 {
                return (0, true);
            }

            // 标记已访问
            g[ux][uy] = 0;

            // 向四周扩展
            let mut closed = true;
            let mut count = 1;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (a, c) = dfs(g, x + dx, y + dy);
                if !c { closed = false; }
                count += a;
            }

            (count, closed)
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let (a, c) = dfs(&mut grid, i as i32, j as i32);
                    if c { ans += a; }
                }
            }
        }

        ans
    }

    // 法二：多源显式栈 + 边界淹没
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // 存放所有与边界连通的陆地
        let mut stack = Vec::new();

        // 收集位于边界上的陆地，入栈并标记为0
        // 从这些边界上的陆地开始淹没
        for j in 0..n {
            if grid[0][j] == 1 {
                grid[0][j] = 0;
                stack.push((0, j));
            }
            if m > 1 && grid[m - 1][j] == 1 {
                grid[m - 1][j] = 0;
                stack.push((m - 1, j));
            }
        }
        for i in 1..m.saturating_sub(1) {
            if grid[i][0] == 1 {
                grid[i][0] = 0;
                stack.push((i, 0));
            }
            if n > 1 && grid[i][n - 1] == 1 {
                grid[i][n - 1] = 0;
                stack.push((i, n - 1));
            }
        }

        const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // 从所有边界陆地触发，淹没所有与之相连的陆地
        while let Some((x, y)) = stack.pop() {
            for (dx, dy) in DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] == 1 {
                    grid[nx][ny] = 0;
                    stack.push((nx, ny));
                }
            }
        }

        // 剩余的 1 都是无法离开边界的飞地
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    ans += 1;
                }
            }
        }

        ans
    }
}