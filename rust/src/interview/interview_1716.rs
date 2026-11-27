struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：动态规划
    pub fn massage(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        else if n == 1 { return nums[0]; }

        // dp[i] 表示 nums[0..=i] 区间内能获得的最长按摩时间
        // 对于第 i 个预约，要么选择要么不选，共两种情况
        // 不选：dp[i] = dp[i - 1]
        // 选择：dp[i] = dp[i - 2] + nums[i]
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..n {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }

        dp[n - 1]
    }

    #[cfg(any())]
    // 法二：动态规划（法一的空间优化版本），双百写法
    pub fn massage(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        else if n == 1 { return nums[0]; }
        else if n == 2 { return nums[0].max(nums[1]); }

        let mut f0 = nums[0];
        let mut f1 = nums[0].max(nums[1]);
        let mut f2 = 0;
        for &x in nums.iter().skip(2) {
            f2 = f1.max(f0 + x);
            f0 = f1;
            f1 = f2;
        }

        f2
    }

    #[cfg(any())]
    // 法三：动态规划（另一种状态定义办法）
    pub fn massage(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        else if n == 1 { return nums[0]; }

        // dp[i][0] 表示 nums[0..=i] 区间内不选第 i 个预约的最长按摩时间
        // dp[i][1] 表示 nums[0..=i] 区间内选择第 i 个预约的最长按摩时间
        let mut dp = vec![vec![0, 0]; n];
        dp[0][1] = nums[0];
        for i in 1..n {
            // 不选第 i 个时，第 i - 1 个可选可不选
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
            // 选择第 i 个时，第 i - 1 个不可选
            dp[i][1] = dp[i - 1][0] + nums[i];
        }

        dp[n - 1][0].max(dp[n - 1][1])
    }

    // 法四：法三的空间优化版本（双百写法）
    pub fn massage(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        else if n == 1 { return nums[0]; }

        let mut f0 = [0, nums[0]];
        let mut f1 = [0, 0];
        for &x in nums.iter().skip(1) {
            f1[0] = f0[0].max(f0[1]);
            f1[1] = f0[0] + x;
            f0 = f1;
        }

        f1[0].max(f1[1])
    }
}