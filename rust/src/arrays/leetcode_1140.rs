struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        // dp[i][m]表示前i堆石头已经被取走，当前M=m的情况
        // 下，接下去取石头的玩家可以比另一方多取的石头数。
        // (1) 当i == n时, dp[0][m] = 0
        // (2) 遍历当前玩家取x = 1 ~ 2m堆石头，并减去对方
        // 玩家多取的石头数，挑选最大值作为最优策略
        let n = piles.len();
        let mut dp = vec![vec![i32::MIN; n + 1]; n + 1];

        for i in (0..=n).rev() {
            for m in 1..=n {
                if i == n {
                    dp[i][m] = 0;
                } else {
                    let mut sum = 0;
                    for x in 1..=(2 * m) {
                        if i + x > n { break; }
                        sum += piles[i + x - 1];
                        let idx = n.min(m.max(x));
                        dp[i][m] = dp[i][m].max(sum - dp[i + x][idx]);
                    }
                }
            }
        }

        let piles_sum: i32 = piles.into_iter().sum();
        (dp[0][1] + piles_sum) / 2
    }
}