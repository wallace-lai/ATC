struct Solution;

impl Solution {
    /// 法一：动态规划
    /// dp[i][j]：当前剩下的石子堆为下标i到j时，当前玩家与另一个玩家的石子数量之差的最大值
    /// 
    /// dp[i][j] = 0, i > j
    /// dp[i][i] = piles[i], i == j
    /// 
    /// 当 i < j时，当前玩家可以选择取走piles[i]或者piles[j]
    /// dp[i][j] = max(piles[i] - dp[i + 1][j], piles[j] - dp[i][j - 1])
    /// 

    // pub fn stone_game(piles: Vec<i32>) -> bool {
    //     let len = piles.len();
    //     let mut dp = vec![vec![0; len]; len];

    //     for i in 0..len {
    //         dp[i][i] = piles[i];
    //     }
    //     for i in (0..=(len - 2)).rev() {
    //         for j in (i + 1)..len {
    //             dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
    //         }
    //     }

    //     dp[0][len - 1] > 0
    // }

    // 法二：数学
    pub fn stone_game(_: Vec<i32>) -> bool {
        true
    }
}