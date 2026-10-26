struct Solution;

impl Solution {
    // 法一：DFS + 边界标记
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        
        // 返回该连通块的价值总数
        // grid[x][y] = 0 - 水域
        // grid[x][y] > 0 - 陆地，其值为价值
        fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> usize {
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;

            // 越界：不贡献价值
            if x < 0 || x >= m || y < 0 || y >= n {
                return 0;
            }

            // 遇到水域，停止搜索，且不贡献价值
            let (ux, uy) = (x as usize, y as usize);
            if grid[ux][uy] == 0 {
                return 0;
            }

            // 保存价值后标记已访问
            let mut value = grid[ux][uy] as usize;
            grid[ux][uy] = 0;

            // 向四周扩展
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                value += dfs(grid, x + dx, y + dy);
            }

            // 返回当前地块的总价值
            value
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    let value = dfs(&mut grid, i as i32, j as i32);
                    if value % k as usize == 0 { count += 1; }
                }
            }
        }

        count
    }
}