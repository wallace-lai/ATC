struct Solution;

use std::collections::VecDeque;

impl Solution {
    // 利用BFS进行染色
    // 1. 将BFS访问过的地方标记已访问，确保每个格子只访问一次
    // 2. 遍历过程中记录面积，如果当前区域与走廊相邻，则这块区域的面积不记入最终结果
    pub fn largest_area(grid: Vec<String>) -> i32 {
        let mut map: Vec<Vec<u8>> = grid.into_iter()
            .map(|row| row.into_bytes())
            .collect();

        // 区域大小：m * n
        let m = map.len();
        let n = map[0].len();
        if m <= 2 || n <= 2 { return 0; }

        // 上下左右四个方向
        let dx = [-1, 1, 0, 0];
        let dy = [0, 0, -1, 1];

        let mut ans = 0;
        let mut q = VecDeque::new();
        
        for r in 0..m {
            for c in 0..n {
                // 当前区域的类型
                let mark = map[r][c];
                if mark == b'0' || mark == b'6' { continue; }

                // 当前区域的面积
                let mut area = 0;

                // 标记当前区块是否和走廊相连
                let mut flag = false;

                q.push_back((r, c));
                map[r][c] = b'6';
                area += 1;

                while !q.is_empty() {
                    let len = q.len();
                    for _ in 0..len {
                        let (x, y) = q.pop_front().unwrap();
                        let (x, y) = (x as i32, y as i32);

                        // 往上下左右四个方向扩展
                        for k in 0..4 {
                            let nx = x + dx[k];
                            let ny = y + dy[k];
                            // 所扩展区域超出边界，说明(x, y)位于场地边缘，相当于当前区块和走廊相连
                            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                                flag = true;
                                continue;
                            }

                            // 所扩展区域为走廊，则当前区块和走廊相连
                            let (nx, ny) = (nx as usize, ny as usize);
                            if map[nx][ny] == b'0' {
                                flag = true;
                                continue;
                            }

                            // 所扩展区域的类型与当前类型不一致，跳过
                            if map[nx][ny] != mark { continue; }

                            // 更新面积
                            area += 1;

                            // 更新访问记录
                            map[nx][ny] = b'6';

                            // 所扩展位置入队列
                            q.push_back((nx, ny));
                        }
                    }
                }

                if flag == false { ans = ans.max(area); }
            }
        }

        ans
    }
}