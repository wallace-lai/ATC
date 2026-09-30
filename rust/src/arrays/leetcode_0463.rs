struct Solution;

impl Solution {
    pub fn start_position(grid: &Vec<Vec<i32>>) -> (usize, usize) {
        let m = grid.len();
        let n = grid[0].len();
        for x in 0..m {
            for y in 0..n {
                if grid[x][y] == 1 {
                    return (x, y);
                }
            }     
        }

        unreachable!()
    }

    // 3ms
    // pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    //     use std::collections::VecDeque;
    //     let m = grid.len();
    //     let n = grid[0].len();
    //     let start = Self::start_position(&grid);

    //     // 上下左右四个方向
    //     let dx = [-1, 1, 0, 0];
    //     let dy = [0, 0, -1, 1];
    
    //     let mut ans = 0;
    //     let mut vis = vec![0_u8; m * n];
    //     let mut q = VecDeque::new();

    //     q.push_back(start);
    //     vis[start.0 * n + start.1] = 1;

    //     while !q.is_empty() {
    //         ans += 4;   // 遇到一个方块，周长增加4
    //         let curr = q.pop_front().unwrap();
    //         let x = curr.0 as i32;
    //         let y = curr.1 as i32;

    //         for i in 0..4 {
    //             let nx = x + dx[i];
    //             let ny = y + dy[i];
    //             if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
    //                 continue;
    //             }

    //             let nx = nx as usize;
    //             let ny = ny as usize;
    //             if grid[nx][ny] == 1 {
    //                 ans -= 1;   // 只要有新方块与当前方块相连，当前方块的边数自动减1
    //                 if vis[nx * n + ny] == 0 {
    //                     // 如果新方块未被访问，则加入队列
    //                     vis[nx * n + ny] = 1;
    //                     q.push_back((nx, ny));
    //                 }
    //             }
    //         }
    //     }

    //     ans
    // }

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        // 上下左右四个方向
        let dx = [-1, 1, 0, 0];
        let dy = [0, 0, -1, 1];

        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;

        for x in 0..m {
            for y in 0..n {
                if grid[x][y] == 1 {
                    ans += 4;
                    
                    for i in 0..4 {
                        let nx = x as i32 + dx[i];
                        let ny = y as i32 + dy[i];
                        if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                            continue;
                        }

                        let nx = nx as usize;
                        let ny = ny as usize;
                        if grid[nx][ny] == 1 {
                            ans -= 1;
                        }
                    }
                }
            }
        }

        ans
    }
}
