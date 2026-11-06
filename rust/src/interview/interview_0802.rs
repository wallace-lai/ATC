struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：递归 DFS + 失败记忆化
    pub fn path_with_obstacles(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
        // 从 (x, y) 处开始往终点搜索，返回当前位置是否可通达终点
        fn dfs(
            x: usize, y: usize, m: usize, n: usize,
            grid: &Vec<Vec<i32>>, dead: &mut Vec<Vec<bool>>,
            path: &mut Vec<Vec<i32>>) -> bool {
            // 越界或者遇到障碍物或者当前位置已被证明不可达
            if x >= m || y >= n || grid[x][y] == 1 || dead[x][y] {
                return false;
            }

            // 刚好到达终点
            if x == m - 1 && y == n - 1 {
                path.push(vec![x as i32, y as i32]);
                return true;
            }

            // 往下或者右探索
            for (nx, ny) in [(x + 1, y), (x, y + 1)] {
                if dfs(nx, ny, m, n, grid, dead, path) {
                    // 两个方向只要有一个方向能通，那么当前位置就是可达的
                    path.push(vec![x as i32, y as i32]);
                    return true;
                }
            }

            // 当前位置不可通达终点
            dead[x][y] = true;
            false
        }

        let m = grid.len();
        let n = grid[0].len();
        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return vec![];
        }

        let mut ans = vec![];
        // dead[i][j] = true 表示从 (i, j) 出发无法到达终点
        let mut dead = vec![vec![false; n]; m];

        if dfs(0, 0, m, n, &grid, &mut dead, &mut ans) {
            ans.reverse();  // 反转才是路径顺序
            ans
        } else {
            vec![]
        }
    }

    #[cfg(any())]
    // 法二：显式栈 DFS + parent映射
    pub fn path_with_obstacles(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return vec![];
        }

        // 记录到达位置 (x, y) 处的父节点位置
        let mut parent = vec![vec![None; n]; m];
        // 访问标记数组，为了防止同一格子被多条不同路径重复入栈
        let mut visited = vec![vec![false; n]; m];

        let mut stack = vec![(0, 0)];
        visited[0][0] = true;

        while let Some((x, y)) = stack.pop() {
            if x == m - 1 && y == n - 1 {
                // 沿着 parent 回溯重建路径
                let mut path = Vec::with_capacity(n + m);
                let (mut x, mut y) = (x, y);
                loop {
                    path.push(vec![x as i32, y as i32]);
                    match parent[x][y] {
                        Some((px, py)) => { x = px; y = py; },
                        None => break,  // 到达了起点
                    }
                }
                path.reverse();
                return path;
            }

            for (nx, ny) in [(x + 1, y), (x, y + 1)] {
                if nx < m && ny < n && !visited[nx][ny] && grid[nx][ny] == 0 {
                    visited[nx][ny] = true;
                    parent[nx][ny] = Some((x, y));
                    stack.push((nx, ny));
                }
            }
        }

        vec![]
    }

    // 法三：BFS + parent映射
    pub fn path_with_obstacles(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let m = grid.len();
        let n = grid[0].len();
        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return vec![];
        }

        // 记录到达位置 (x, y) 处的父节点位置
        let mut parent = vec![vec![None; n]; m];
        // 访问标记数组
        let mut visited = vec![vec![false; n]; m];

        let mut q = VecDeque::new();
        visited[0][0] = true;
        q.push_back((0, 0));

        while let Some((x, y)) = q.pop_front() {
            if x == m - 1 && y == n - 1 {
                // 沿着 parent 回溯重建路径
                let mut path = Vec::with_capacity(n + m);
                let (mut x, mut y) = (x, y);
                loop {
                    path.push(vec![x as i32, y as i32]);
                    match parent[x][y] {
                        Some((px, py)) => { x = px; y = py; },
                        None => break,  // 到达了起点
                    }
                }
                path.reverse();
                return path;
            }

            for (nx, ny) in [(x + 1, y), (x, y + 1)] {
                if nx < m && ny < n && !visited[nx][ny] && grid[nx][ny] == 0 {
                    visited[nx][ny] = true;
                    parent[nx][ny] = Some((x, y));
                    q.push_back((nx, ny));
                }
            }
        }

        vec![]
    }
}