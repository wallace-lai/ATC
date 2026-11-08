struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：并查集（杀鸡用牛刀）
    pub fn count_servers(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();


        let mut id = 1;
        let mut count = 0usize;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = id;
                    id += 1;
                    count += 1;
                }
            }
        }

        let mut parent: Vec<usize> = (0..=count).collect();
        let mut size: Vec<usize> = vec![1; count + 1];

        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            let mut root = x;
            while parent[root] != root {
                root = parent[root];
            }

            while parent[x] != root {
                let next = parent[x];
                parent[x] = root;
                x = next;
            }

            root
        }

        fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
            let (mut rx, mut ry) = (find(parent, x), find(parent, y));
            if rx == ry { return; }

            if size[rx] < size[ry] {
                std::mem::swap(&mut rx, &mut ry);
            }

            parent[ry] = rx;
            size[rx] += size[ry];
            size[ry] = 0;   // y合并到x以后，清空y集合的个数
        }

        // 按行合并
        for i in 0..m {
            let mut p: Option<usize> = None;
            for j in 0..n {
                if grid[i][j] >= 1 {
                    match p {
                        Some(x) => { union(&mut parent, &mut size, grid[i][x] as usize, grid[i][j] as usize) },
                        None => { p = Some(j) },
                    }
                }
            }
        }

        // 按列合并
        for j in 0..n {
            let mut p: Option<usize> = None;
            for i in 0..m {
                if grid[i][j] >= 1 {
                    match p {
                        Some(x) => { union(&mut parent, &mut size, grid[x][j] as usize, grid[i][j] as usize); },
                        None => { p = Some(i); },
                    }
                }
            }
        }

        size.iter()
            .filter(|s| **s > 1)
            .map(|s| *s as i32)
            .sum()
    }

    // 法二：直接遍历统计
    pub fn count_servers(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_cnt = vec![0; m];
        let mut col_cnt = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    row_cnt[i] += 1;
                    col_cnt[j] += 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && (row_cnt[i] > 1 || col_cnt[j] > 1) {
                    ans += 1;
                }
            }
        }

        ans
    }
}