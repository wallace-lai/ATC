struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：动态规划
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = arr.len();
        if n == 1 { return if arr[0] & 1 == 1 { 1 } else { 0 }; }

        // dp[i][0] 表示以 arr[i] 结尾且和为偶数的子数组数目
        // dp[i][1] 表示以 arr[i] 结尾且和为奇数的子数组数目
        let mut dp = vec![[0, 0]; n];
        if arr[0] & 1 == 1 {
            dp[0][1] = 1;
        } else {
            dp[0][0] = 1;
        }
        let mut ans = dp[0][1];

        for (i, &x) in arr.iter().enumerate().skip(1) {
            if x & 1 == 1 {
                dp[i][0] = dp[i - 1][1];        // 偶 = 奇 + 奇
                dp[i][1] = (dp[i - 1][0] + 1) % MOD;    // 奇 = 偶 + 奇
            } else {
                dp[i][0] = (dp[i - 1][0] + 1) % MOD;    // 偶 = 偶 + 偶
                dp[i][1] = dp[i - 1][1];        // 奇 = 奇 + 偶
            }
            ans += dp[i][1];
            ans %= MOD;
        }

        ans
    }

    #[cfg(any())]
    // 法二：动态规划（空间优化）
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = arr.len();
        if n == 1 { return if arr[0] & 1 == 1 { 1 } else { 0 }; }

        let mut f0 = [0, 0];
        let mut f1 = [0, 0];
        if arr[0] & 1 == 1 {
            f0[1] = 1;
        } else {
            f0[0] = 1;
        }
        let mut ans = f0[1];

        for &x in arr.iter().skip(1) {
            if x & 1 == 1 {
                f1[0] = f0[1];
                f1[1] = (f0[0] + 1) % MOD;
            } else {
                f1[0] = (f0[0] + 1) % MOD;
                f1[1] = f0[1];
            }

            ans += f1[1];
            ans %= MOD;
            f0 = f1;
        }

        ans
    }

    // 法三：前缀和计数器
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        // count[0] 表示已出现的偶数前缀和个数
        // count[1] 表示已出现的奇数前缀和个数
        // count[0] = 1 表示空前缀
        let mut count = [1, 0];

        let mut s = 0usize;     // 当前前缀和的奇偶性（0 = 偶、1 = 奇）
        let mut ans = 0;

        for &x in arr.iter() {
            // s 和 x 不同时，奇偶性才会改变
            s ^= (x & 1) as usize;

            // 与相反奇偶性的历史前缀配对，才能构成奇数和的子数组
            ans += count[s ^ 1];
            ans %= MOD;

            // 记录当前前缀
            count[s] += 1;
            count[s] %= MOD;
        }

        ans
    }
}