struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：动态规划
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        
        // 返回严格 LIS 的长度
        fn lis(nums: &[i32]) -> usize {
            let n = nums.len();
            if n == 0 { return 0; }

            // dp[i] 为以 nums[i] 结尾的 LIS 长度
            let mut dp = vec![1; n];
            let mut ans = 1;

            for i in 0..n {
                for j in 0..i {
                    // 如果是求最长非降子序列长度，则
                    // if nums[j] <= nums[i] {
                    if nums[j] < nums[i] {
                        dp[i] = dp[i].max(dp[j] + 1);
                    }
                }
                ans = ans.max(dp[i]);
            }

            ans
        }

        lis(&nums) as i32
    }

    #[cfg(any())]
    // 法二：贪心 + 二分
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        
        // 返回严格 LIS 的长度
        fn lis(nums: &[i32]) -> usize {
            // tails[i] 为所有长度为 i+1 的上升子序列中，结尾元素的最小值
            let mut tails = Vec::new();

            for &x in nums {
                // 如果是求非降 LIS 的长度，则是
                // let i = tails.partition_point(|&v| v <= x);
    
                // 第一个 >= x 的位置（lower_bound）
                let i = tails.partition_point(|&v| v < x);
                if i == tails.len() {
                    tails.push(x);
                } else {
                    tails[i] = x;
                }
            }

            tails.len()
        }

        lis(&nums) as i32
    }

    // 法三：贪心 + 二分（手写 lower_bound）
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        
        // 找到第一个 >=t 的位置
        fn lower_bound(nums: &[i32], t: i32) -> usize {
            // 左闭右开区间
            let (mut l, mut r) = (0, nums.len());
            while l < r {
                let mid = l + (r - l) / 2;
                if nums[mid] >= t {
                    r = mid;        // mid可能是答案，但还需往前找
                } else {
                    l = mid + 1;    // mid和左边都不满足 >= t
                }
            }
            l   // l == nums.len()时，表示 >=t 位置不存在
        }

        fn lis(nums: &[i32]) -> usize {
            // tails[i] 为所有长度为 i+1 的上升子序列中，结尾元素的最小值
            let mut tails = Vec::new();

            for &x in nums {
                // 第一个 >= x 的位置（lower_bound）
                let i = lower_bound(&tails, x);
                if i == tails.len() {
                    tails.push(x);
                } else {
                    tails[i] = x;
                }
            }

            tails.len()
        }

        lis(&nums) as i32
    }
}