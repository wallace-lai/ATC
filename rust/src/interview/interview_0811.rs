struct Solution;

impl Solution {
    // 法一：动态规划
    pub fn ways_to_change(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const COINS: [usize; 4] = [25, 10, 5, 1];

        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for coin in COINS {
            for j in coin..=n {
                dp[j] = (dp[j] + dp[j - coin]) % MOD;
            }
        }

        dp[n] as i32
    }
}