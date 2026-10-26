struct Solution;

impl Solution {
    // 法一：DFS
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        
        // 标记和 (x, y) 相连的所有连通块并返回整个地块的面积（格子数）
        // 0 - 水域
        // 1 - 陆地
        fn dfs(g: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界，不贡献面积
            if x < 0 || x >= m || y < 0 || y >= n {
                return 0;
            }

            // 遇到水域，停止搜索，不贡献面积
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] == 0 {
                return 0;
            }

            // 标记已访问
            g[ux][uy] = 0;

            // 向四周扩展
            let mut area = 1;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                area += dfs(g, x + dx, y + dy);
            }

            area
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let area = dfs(&mut grid, i as i32, j as i32);
                    ans = ans.max(area);
                }
            }
        }

        ans
    }
}