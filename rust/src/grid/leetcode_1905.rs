struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：递归 DFS + 哈希表
    // 性能很差，没有必要使用哈希表
    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        // 标记当前的陆地并将陆地的所有网格存入映射表中
        // g[x][y] = 0 - 水域
        // g[x][y] = 1 - 陆地
        fn build_map(g: &mut Vec<Vec<i32>>, x: i32, y: i32, id: i32, map: &mut HashMap<(i32, i32), i32>) {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界：直接返回
            if x < 0 || x >= m || y < 0 || y >= n {
                return;
            }

            // 遇到水域，停止搜索
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] == 0 {
                return;
            }

            // 标记 (x, y) 为已访问且其所属的陆地号为 id
            map.insert((x, y), id);
            g[ux][uy] = 0;

            // 向四周扩展
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                build_map(g, x + dx, y + dy, id, map);
            }
        }

        // 递归检查当前陆地是否位于 grid1 的同一块陆地中
        fn check(g: &mut Vec<Vec<i32>>, x: i32, y: i32, id: i32, map: &HashMap<(i32, i32), i32>) -> bool {
            let m = g.len() as i32;
            let n = g[0].len() as i32;

            // 越界：无需检查返 true 即可
            if x < 0 || x >= m || y < 0 || y >= n {
                return true;
            }

            // 遇到水域，停止搜索并返 true
            let (ux, uy) = (x as usize, y as usize);
            if g[ux][uy] == 0 {
                return true;
            }

            // 标记为已访问
            g[ux][uy] = 0;

            let mut result = true;

            // 检查 (x, y) 在 grid1 中是否也为陆地且陆地号和传入的 id 一致
            if map.get(&(x, y)).unwrap_or(&-1) != &id {
                // 检查失败，注意这里不能提前返回
                result = false;
            }

            // 向四周扩展
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if !check(g, x + dx, y + dy, id, map) {
                    result = false;
                }
            }

            // 返回当前陆块的检查情况
            result
        }

        let m = grid1.len();
        let n = grid1[0].len();

        // (x, y) -> id
        // 针对 grid1 中各岛屿所组成的地块坐标到岛屿号的映射
        let mut map = HashMap::new();
        let mut id = 0;

        for i in 0..m {
            for j in 0..n {
                if grid1[i][j] == 1 {
                    build_map(&mut grid1, i as i32, j as i32, id, &mut map);
                    id += 1;
                }
            }
        }

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    if let Some(id) = map.get(&(i as i32, j as i32)) {
                        if check(&mut grid2, i as i32, j as i32, *id, &map) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    // 法二：迭代 DFS
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid2.len();
        let n = grid2[0].len();
        let mut count = 0;
        let mut stk = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 0 { continue; }

                // 入栈前标记已访问
                grid2[i][j] = 0;
                stk.clear();
                stk.push((i, j));
                let mut is_sub_island = true;

                while let Some((x, y)) = stk.pop() {
                    // 只要 (x, y) 在 grid1 中是水域，则该 grid2 岛屿就不是子岛
                    if grid1[x][y] == 0 {
                        // 注意这里不能提前退出循环，岛屿所有陆地标记完毕才能退出
                        is_sub_island = false;
                    }

                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        // 越界
                        if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize {
                            continue;
                        }

                        let (nx, ny) = (nx as usize, ny as usize);
                        if grid2[nx][ny] == 1 {
                            grid2[nx][ny] = 0;  // 入栈前就要标记已访问
                            stk.push((nx, ny));
                        }
                    }
                }

                if is_sub_island { count += 1; }
            }
        }

        count
    }
}