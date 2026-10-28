struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一
    // 存在以下几个可优化的点：
    // 1. 如果 grid[row][col] == color，无需操作，直接返回
    // 2. grid.clone()没必要，使用 bool 型访问数组即可
    // 3. DFS 和边界判断可以合并，避免二次遍历，在遍历过程中只收集边界点
    // 4. 递归 DFS 改为显式栈
    // 5. 边界判断可以统一为四邻接检查
    // 6. 闭包 make_color 可内联
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        
        // 找出所有和 (x, y) 相连的连通分量
        fn dfs(g: &mut Vec<Vec<i32>>, c: i32, x: i32, y: i32, v: &mut Vec<(i32, i32)>) {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界
            if x < 0 || x >= m || y < 0 || y >= n {
                return;
            }

            // 遇到不同颜色的方块，停止搜索
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] != c {
                return;
            }

            // 标记已访问并加入列表
            g[ux][uy] = c + 1;
            v.push((x, y));

            // 向四周扩展
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                dfs(g, c, x + dx, y + dy, v);
            }
        }

        let mut g = grid.clone();
        let mut v = Vec::new();
        let c = grid[row as usize][col as usize];
        dfs(&mut g, c, row, col, &mut v);

        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let mut make_color = |x: i32, y: i32| {
            let mut is_border = false;
            let (ux, uy) = (x as usize, y as usize);

            // 位于边界上
            if x == 0 || x == m - 1 || y == 0 || y == n - 1 {
                is_border = true;
            } else {
                // 与不相同颜色的方块相连
                let c = grid[ux][uy];
                if grid[ux - 1][uy] != c || grid[ux + 1][uy] != c ||
                    grid[ux][uy - 1] != c || grid[ux][uy + 1] != c {
                    is_border = true;
                }
            }

            if is_border { grid[ux][uy] = color; }
        };

        for p in v {
            make_color(p.0, p.1);
        }

        grid
    }

    // 法二：性能优化版本
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let r = row as usize;
        let c = col as usize;
        let orig = grid[r][c];

        // 颜色相同，无需任何操作
        if orig == color { return grid; }

        let mut visited = vec![vec![false; n]; m];
        let mut stack = Vec::new();
        let mut borders = Vec::new();

        visited[r][c] = true;
        stack.push((r, c));

        const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((x, y)) = stack.pop() {
            let mut is_border = false;

            for &(dx, dy) in &DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // 越界：说明 (x, y) 一定是边界
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    is_border = true;
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] != orig {
                    // 邻居颜色不同：说明 (x, y) 是边界
                    is_border = true;
                } else if !visited[nx][ny] {
                    // 同色且未访问：继续遍历
                    visited[nx][ny] = true;
                    stack.push((nx, ny));
                }
            }

            if is_border { borders.push((x, y)); }
        }

        // 只对边界点进行着色
        for (x, y) in borders {
            grid[x][y] = color;
        }

        grid
    }
}