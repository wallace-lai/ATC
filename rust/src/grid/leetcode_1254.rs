struct Solution;

impl Solution {
    // 法一：DFS + 边界标记法
    // 1. 遍历网格，每遇到一个未访问的 0（陆地），就启动一次DFS
    // 2. DFS往四个方向扩展，如果任何一个方向越界，就说明当前岛屿非封闭，标记为false
    // 3. 如果整个岛屿的所有陆地都未越界，它就是封闭的，计数加1
    // 4. 为避免重复访问，在DFS过程中将遍历过的 0（陆地）标记为 1，或者使用visited数组
    // pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {

    //     // 判断当前的陆地是否为封闭岛屿
    //     // grid[x][y] == 0 - 陆地
    //     // grid[x][y] == 1 - 水域
    //     fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    //         let m = grid.len() as i32;
    //         let n = grid[0].len() as i32;

    //         // 越界：说明触碰边界，不是封闭岛屿
    //         if x < 0 || x >= m || y < 0 || y >= n {
    //             return false;
    //         }

    //         // 遇到水域，停止搜索
    //         let (ux, uy) = (x as usize, y as usize);
    //         if grid[ux][uy] == 1 {
    //             return true;
    //         }

    //         // 标记已访问（也可使用visited数组）
    //         grid[ux][uy] = 1;

    //         // 到目前为止，当前地块 (x, y) 是封闭的，但还需要往四周搜索
    //         let mut ans = true;
    //         for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
    //             // 注意不能提前return，必须遍历完整个岛屿以完成标记
    //             if !dfs(grid, x + dx, y + dy) {
    //                 ans = false;
    //             }
    //         }

    //         // 返回当前岛屿的封闭情况
    //         ans
    //     }

    //     // 网格至少得是 1x1 大小
    //     let m = grid.len();
    //     let n = grid[0].len();
    //     let mut count = 0;

    //     for i in 0..m {
    //         for j in 0..n {
    //             if grid[i][j] == 0 &&
    //                 dfs(&mut grid, i as i32, j as i32) {
    //                 count += 1;
    //             }
    //         }
    //     }

    //     count
    // }

    // 法二：DFS/BFS + 先外后内逆向法
    // 思路：所有与边界相连的陆地都不属于封闭岛屿，可以先“淹没”这些边界陆地，剩下的就是封闭岛屿
    // 处理：
    // 1. 预处理：遍历网格的所有边界，一旦遇到 0（陆地），以它为起点进行DFS/BFS，将所有与边界
    //      相连的 0（陆地）淹没（修改为 1（水域））
    // 2. 计数：遍历网格内部，每遇到一个 0（陆地），就说明找到一个封闭岛屿，计数加一，并同样用
    //      DFS/BFS淹没整个岛屿
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        
        // 将 0（陆地）淹没
        // grid[x][y] == 0 - 陆地
        // grid[x][y] == 1 - 水域
        fn flood(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) {
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;

            // 越界
            if x < 0 || x >= m || y < 0 || y >= n {
                return;
            }

            // 遇到水域，停止搜索
            let (ux, uy) = (x as usize, y as usize);
            if grid[ux][uy] == 1 {
                return;
            }

            // 淹没当前地块
            grid[ux][uy] = 1;

            // 向四周扩展淹没
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                flood(grid, x + dx, y + dy);
            }
        }

        // 网格至少得是 1x1 大小
        let m = grid.len();
        let n = grid[0].len();

        // 1. 淹没和边界相连的陆地
        for i in 0..m {
            for j in 0..n {
                if (i == 0 || i == m - 1 || j == 0 || j == n - 1) &&
                    grid[i][j] == 0 {
                    flood(&mut grid, i as i32, j as i32);
                }
            }
        }

        // 2. 统计内部剩余的封闭岛屿数
        let mut count = 0;
        // 使用saturating_sub避免减法溢出
        for i in 1..m.saturating_sub(1) {
            for j in 1..n.saturating_sub(1) {
                if grid[i][j] == 0 {
                    count += 1;
                    // 完成计数后需要将整个岛屿淹没
                    flood(&mut grid, i as i32, j as i32);
                }
            }
        }

        count
    }
}
