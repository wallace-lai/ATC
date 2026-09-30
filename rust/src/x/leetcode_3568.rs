struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_moves(map: Vec<String>, energy: i32) -> i32 {
        let m = map.len();
        let n = map[0].len();
        let dx = [-1, 1, 0, 0];
        let dy = [0, 0, -1, 1];

        // 获取起始位置 (sx, sy) 并给所有的垃圾编号
        let mut seq = 0;
        let (mut sx, mut sy) = (0, 0);
        let mut id = vec![vec![0; n]; m];
        for i in 0..m {
            let row = map[i].as_bytes();
            for j in 0..n {
                if row[j] == b'S' {
                    (sx, sy) = (i, j);
                } else if row[j] == b'L' {
                    id[i][j] = 1 << seq;
                    seq += 1;
                }
            }
        }

        let full = 1 << seq;
        let mut best_energy = vec![vec![vec![-1; full]; n]; m];

        #[derive(Clone, Copy)]
        struct Info {
            x: usize,
            y: usize,
            mask: usize,
            energy: i32,
            steps: i32
        }

        let mut q: VecDeque<Info> = VecDeque::new();
        q.push_back(Info {
            x: sx, y: sy, mask: 0, energy: energy, steps: 0
        });

        while !q.is_empty() {
            // while循环条件保证队列不为空
            let curr = q.pop_front().unwrap();
            if curr.mask == full - 1 { return curr.steps; }
            if curr.energy == 0 { continue; }
            for k in 0..4 {
                let nx = curr.x as i32 + dx[k];
                let ny = curr.y as i32 + dy[k];
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let c = map[nx].as_bytes()[ny];
                if c == b'X' { continue; }
                let ne = if c == b'R' { energy } else { curr.energy - 1 };
                let nmask = curr.mask | id[nx][ny] as usize;
                if ne > best_energy[nx][ny][nmask] {
                    best_energy[nx][ny][nmask] = ne;
                    q.push_back(Info {
                        x: nx, y: ny, mask: nmask, energy: ne, steps: curr.steps + 1
                    });
                }
            }
        }

        -1
    }
}