struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：分治 (O(nlogn))
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        // 求 nums[l..=r] 范围内的最大子数组和
        fn solve(nums: &[i32], l: usize, r: usize) -> i32 {
            if l == r { return nums[l]; }

            let m = l + (r - l) / 2;
            // 完全在左半边
            let left_max = solve(nums, l, m);
            // 完全在右半边
            let right_max = solve(nums, m + 1, r);
            // 跨越中点
            let cross_max = cross_sum(nums, l, m, r);

            left_max.max(right_max).max(cross_max)
        }

        // 跨中点的最大子数组和，必包含 nums[m] 和 nums[m + 1]
        fn cross_sum(nums: &[i32], l: usize, m: usize, r: usize) -> i32 {
            // 从中点往左扫，求以 m 结尾的最大后缀和
            let mut sum = 0;
            let mut left_suffix = i32::MIN;
            for i in (l..=m).rev() {
                sum += nums[i];
                left_suffix = left_suffix.max(sum);
            }

            // 从中点+1向右扫，求以 m+1 开头的最大前缀和
            let mut sum = 0;
            let mut right_prefix = i32::MIN;
            for i in (m + 1)..=r {
                sum += nums[i];
                right_prefix = right_prefix.max(sum);
            }

            left_suffix + right_prefix
        }

        if nums.is_empty() { return 0; }
        solve(&nums, 0, nums.len() - 1)
    }

    #[cfg(any())]
    // 法二：动态规划 (O(n))
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // f[i] 表示前 i 个数的最大连续子序列的和，对于第 i 个
        // 数要么选择加入前面的子序列，要么与前面的子序列断开，所以
        // 状态转移方程为：f[i] = max(nums[i], f[i - 1] + nums[i])
        let n = nums.len();
        if n == 0 { return 0; }

        let mut f = vec![0; n];
        f[0] = nums[0];
        let mut ans = nums[0];

        for i in 1..n {
            f[i] = nums[i].max(f[i - 1] + nums[i]);
            ans = ans.max(f[i]);
        }

        ans
    }

    // 法三：动态规划 + 空间优化
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }

        let mut f0 = nums[0];
        let mut ans = nums[0];
        let mut f1;

        for i in 1..n {
            f1 = nums[i].max(f0 + nums[i]);
            ans = ans.max(f1);
            f0 = f1;
        }
        ans
    }
}