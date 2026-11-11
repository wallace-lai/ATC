struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：位运算 + 回溯
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut ans = Vec::new();
        let mut queens = vec![0usize; n];

        fn dfs(
            n: usize,
            row: usize,
            col: u32,   // 第 c 位为 1 表示第 c 列被占
            d1: u32,    // 第 i 位为 1 表示 r - c + n == i 的主对角线被占
            d2: u32,    // 第 i 位位 1 表示 r + c == i 的副对角线被占
            queens: &mut Vec<usize>,
            ans: &mut Vec<Vec<String>>
        ) {
            if row == n {
                ans.push(
                    queens.iter().map(|&c| {
                        let mut s = vec!['.'; n];
                        s[c] = 'Q';
                        s.into_iter().collect()
                    }).collect()
                );
                return;
            }

            for c in 0..n {
                let bit = 1u32 << c;
                let b1 = 1u32 << (row + n - c); // r - c + n
                let b2 = 1u32 << (row + c);     // r + c
                if col & bit != 0 ||    // 该列已有皇后，冲突
                    d1 & b1 != 0 ||     // 主对角线被占，冲突
                    d2 & b2 != 0 {      // 副对角线被占，冲突
                    continue;
                }
                queens[row] = c;
                dfs(n, row + 1, col | bit, d1 | b1, d2 | b2, queens, ans);
            }
        }

        dfs(n, 0, 0, 0, 0, &mut queens, &mut ans);
        ans
    }

    // 法二：性能优化版（跳过了对不可用列的循环）
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut ans = Vec::new();
        let mut queens = vec![0usize; n];

        fn dfs(
            n: usize,
            row: usize,
            col: u32,   // 第 c 位为 1 表示第 c 列被占
            d1: u32,    // 第 c 位为 1 表示第 c 列所在主对角线被占
            d2: u32,    // 第 c 位为 1 表示第 c 列所在副对角线被占
            queens: &mut Vec<usize>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                ans.push(
                    queens.iter().map(|&c| {
                        let mut s = vec!['.'; n];
                        s[c] = 'Q';
                        s.into_iter().collect()
                    }).collect()
                );
                return;
            }

            let mut avail = !(col | d1 | d2) & ((1u32 << n) - 1);
            while avail != 0 {
                let bit = avail & avail.wrapping_neg(); // 取最低位的 1
                avail &= avail - 1;                          // 清除最低位的 1

                let c = bit.trailing_zeros() as usize;
                queens[row] = c;

                // d1 每下一行左移，d2 每下一行右移，省去绝对索引计算
                dfs(n, row + 1, col | bit, (d1 | bit) << 1, (d2 | bit) >> 1, queens, ans);
            }
        }

        dfs(n, 0, 0, 0, 0, &mut queens, &mut ans);
        ans
    }
}