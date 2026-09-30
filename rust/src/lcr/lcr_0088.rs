struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = vec![0; len + 1];
        dp[2] = cost[0].min(cost[1]);
        for i in 3..=len {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        }

        dp[len]
    }
}