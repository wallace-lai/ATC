struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：DFS + 边界标记
    pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
        
        // 标记所有的连通子块
        // board[x][y] = 'X' - 舰队
        // board[x][y] = '.' - 空地
        fn dfs(board: &mut Vec<Vec<char>>, x: i32, y: i32) {
            let m = board.len() as i32;
            let n = board[0].len() as i32;

            // 越界：直接返回即可
            if x < 0 || x >= m || y < 0 || y >= n {
                return;
            }

            // 遇到空地，停止搜索
            let (ux, uy) = (x as usize, y as usize);
            if board[ux][uy] == '.' {
                return;
            }

            // 标记已访问
            board[ux][uy] = '.';

            // 向四周扩展，题目保证了舰队之间有空地分隔
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                dfs(board, x + dx, y + dy);
            }
        }

        let m = board.len();
        let n = board[0].len();
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'X' {
                    count += 1;
                    dfs(&mut board, i as i32, j as i32);
                }
            }
        }

        count
    }

    // 法二：直接扫描即可，不修改 board 且 O(1) 的空间复杂度
    // 题目保证两个舰队之间至少隔了一个空格，如果一个战舰的左边和上边都不是战舰
    // 那么这个战舰就是一个舰队的战舰头，统计战舰头的数量就是舰队的数量
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let m = board.len();
        let n = board[0].len();
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'X' {
                    // 左边没有战舰或者已在最左的列
                    let no_left = j == 0 || board[i][j - 1] != 'X';
                    // 上边没有战舰或者已在最上的行
                    let no_up = i == 0 || board[i - 1][j] != 'X';
                    if no_left && no_up {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}