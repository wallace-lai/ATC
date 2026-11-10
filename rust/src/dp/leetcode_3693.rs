struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：动态规划
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;

        // dp[i] 表示从 0 到 i 的最小总成本
        let mut dp = vec![0; n + 1];

        for i in 1..=n as i32 {
            let mut tmp = i32::MAX;
            let j = std::cmp::max(i - 3, 0);
            for k in j..i {
                tmp = tmp.min(dp[k as usize] + (i - k) * (i - k));
            }
            dp[i as usize] = tmp + costs[i as usize - 1];
        }

        dp[n]
    }

    // 法二：动态规划 + 空间优化
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let (mut f0, mut f1, mut f2) = (0, 0, 0);
        for c in costs {
            let tmp = std::cmp::min(
                std::cmp::min(f0 + 9, f1 + 4),
                f2 + 1
            ) + c;

            f0 = f1;
            f1 = f2;
            f2 = tmp;
        }
        f2
    }
}