struct Solution;

impl Solution {
    // 法一：DFS
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        
        // 标记和 (x, y) 相连的所有连通块
        // '0' - 水域
        // '1' - 陆地
        fn dfs(g: &mut Vec<Vec<char>>, x: i32, y: i32) {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界
            if x < 0 || x >= m || y < 0 || y >= n {
                return;
            }

            // 遇到水域，停止搜索
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] == '0' {
                return;
            }

            // 标记当前地块已访问
            g[ux][uy] = '0';

            // 向四周扩展
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                dfs(g, x + dx, y + dy);
            }
        }

        let m = grid.len();
        let n = grid[0].len();

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i as i32, j as i32);
                    count += 1;
                }
            }
        }

        count
    }
}