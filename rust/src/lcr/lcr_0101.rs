struct Solution;

impl Solution {
    // pub fn dfs(nums: &Vec<i32>, i: usize, target: i32, ans: &mut bool) {
    //     if i >= nums.len() { return; }
    //     if *ans == true { return; }
    //     if target == 0 { *ans = true; return; }

    //     // 选择nums[i]
    //     if target >= nums[i] {
    //         Self::dfs(nums, i + 1, target - nums[i], ans);
    //     }

    //     // 不选择nums[i]
    //     Self::dfs(nums, i + 1, target, ans);
    // }

    // 超时
    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let sum: i32 = nums.iter().sum();
    //     if sum & 1 == 1 { return false; }

    //     let mut ans = false;
    //     Self::dfs(&nums, 0, sum / 2, &mut ans);
    //     ans
    // }

    // 21ms
    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let n = nums.len();
    //     if n < 2 { return false; }
    //     let sum: i32 = nums.iter().sum();
    //     if sum & 1 == 1 { return false; }

    //     let t = sum as usize / 2;
    //     let max = nums.iter().max().unwrap();
    //     if *max as usize > t { return false; }

    //     // dp[i][j]表示从数组的[0, i]下标范围内选取若干个正整数，是否
    //     // 存在一种选取方案使得被选取的正整数的和等于j，初始时，dp中全为false
    //     let mut dp = vec![vec![false; t + 1]; n];

    //     // 转移方程：
    //     // j == 0时，dp[i][0] = true
    //     // i == 0时，dp[0][nums[0]] = true
    //     // i > 0, j > 0时，
    //     // （1）如果j >= nums[i]，则对于nums[i]可以选取也可以不选取
    //     // （2）如果j < nums[i]，则无法选取nums[i]
    //     for i in 0..n { dp[i][0] = true; }
    //     dp[0][nums[0] as usize] = true;

    //     for i in 1..n {
    //         let num = nums[i] as usize;
    //         for j in 1..=t {
    //             if j >= num {
    //                 dp[i][j] = dp[i - 1][j] | dp[i - 1][j - num];
    //             } else {
    //                 dp[i][j] = dp[i - 1][j];
    //             }
    //         }
    //     }

    //     dp[n - 1][t]
    // }

    // 8ms
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 { return false; }
        let sum: i32 = nums.iter().sum();
        if sum & 1 == 1 { return false; }

        let t = sum as usize / 2;
        let max = nums.iter().max().unwrap();
        if *max as usize > t { return false; }

        let mut dp = vec![false; t + 1];
        dp[0] = true;

        // 当前已处理元素能达到的最大和
        let mut max_sum = 0;
        for &num in &nums {
            max_sum = std::cmp::min(t, num as usize + max_sum);
            for j in (num as usize..=max_sum).rev() {
                if dp[j - num as usize] { dp[j] = true; }
            }

            if dp[t] { return true; }
        }

        dp[t]
    }
}