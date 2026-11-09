struct Solution;

impl Solution {
    // 法一：带权三维偏序最长链
    // 思路：
    // 1. 按某一维度排序（比如 w 升序，w 相同按 d 升序，再相同按 h 升序）
    // 2. dp[i] 为以第 i 个箱子为塔底时能堆出的最大高度
    // 3. 转移：对所有 j<i，如果 b[j] 三个维度都严格小于 b[i]，则可以将
    // b[i] 放到 b[j] 上面
    pub fn pile_box(mut b: Vec<Vec<i32>>) -> i32 {
        let n = b.len();
        if n == 0 { return 0; }

        // 排序：按 w 升序，w 相同按 d 升序，再相同按 h 升序
        b.sort_by(|x, y| {
            x[0].cmp(&y[0])
                .then(x[1].cmp(&y[1]))
                .then(x[2].cmp(&y[2]))
        });

        let mut dp = vec![0; n];
        let mut ans = 0;

        for i in 0..n {
            dp[i] = b[i][2];    // 至少能单独放这一个箱子
            for j in 0..i {
                // 三个都严格小，才能放 i 上面
                if b[j][0] < b[i][0] &&
                    b[j][1] < b[i][1] &&
                    b[j][2] < b[i][2] {
                    dp[i] = dp[i].max(dp[j] + b[i][2]);
                }
            }
            ans = ans.max(dp[i]);
        }

        ans
    }
}