

struct Solution;

impl Solution {
    // 法一：BFS
    // 性能差，可优化点有：
    // 1. 只求面积，不需要按层遍历，没有必要对 BFS 进行分层
    // 2. 显式 DFS 通常比 VecDeque BFS 常数更小
    // 3. 复用容器，避免每个水塘都新建一个 VecDeque
    // 4. 坐标用 usize 以减少转换
    #[cfg(any())]
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;

        // 标记和 (x, y) 相连的水域并返回整个水域的面积（格子数）
        // =0 - 水域
        // >0 - 陆地
        // 使用 BFS 以避免 DFS 堆栈深度爆炸
        fn bfs(land: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
            let m = land.len() as i32;
            let n = land[0].len() as i32;
            const DIR: [(i32, i32); 8] = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1), (0, 1),
                (1, -1), (1, 0), (1, 1)
            ];

            // 入队列前标记已访问
            let (ux, uy) = (x as usize, y as usize);
            land[ux][uy] = 1;

            let mut area = 0;
            let mut q = VecDeque::new();
            q.push_back((x, y));

            while !q.is_empty() {
                let len = q.len();
                for _ in 0..len {
                    let (x, y) = q.pop_front().unwrap();
                    area += 1;

                    for (dx, dy) in DIR {
                        let nx = x + dx;
                        let ny = y + dy;
                        // 越界
                        if nx < 0 || nx >= m || ny < 0 || ny >= n {
                            continue;
                        }
                        // 遇到陆地
                        let (nx, ny) = (nx as usize, ny as usize);
                        if land[nx][ny] > 0 {
                            continue;
                        }
                        // 入队列前标记已访问
                        land[nx][ny] = 1;

                        q.push_back((nx as i32, ny as i32));
                    }
                }
            }

            area
        }

        let m = land.len();
        let n = land[0].len();
        let mut ans = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 0 {
                    let area = bfs(&mut land, i as i32, j as i32);
                    ans.push(area);
                }
            }
        }

        ans.sort_unstable();
        ans
    }

    // 法二：显式 DFS
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let m = land.len();
        let n = land[0].len();
        let mut ans = Vec::new();

        // 复用一个显示栈，避免重新分配
        let mut stk = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if land[i][j] != 0 { continue; }

                // 入栈前标记，避免重复入栈
                land[i][j] = 1;
                stk.push((i, j));

                let mut area = 0;
                
                while let Some((x, y)) = stk.pop() {
                    area += 1;

                    let x0 = x.saturating_sub(1);
                    let x1 = (x + 1).min(m - 1);
                    let y0 = y.saturating_sub(1);
                    let y1 = (y + 1).min(n - 1);

                    for nx in x0..=x1 {
                        for ny in y0..=y1 {
                            if land[nx][ny] == 0 {
                                land[nx][ny] = 1;
                                stk.push((nx, ny));
                            }
                        }
                    }
                }

                ans.push(area);
            }
        }

        ans.sort_unstable();
        ans
    }
}