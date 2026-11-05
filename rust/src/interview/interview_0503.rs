struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：位运算
    pub fn reverse_bits(num: i32) -> i32 {
        // 特例
        if num == -1 { return 32; }

        let mut ans = 0;
        let mut curr = 0;   // 当前连续 1 的个数
        let mut prev = 0;   // 上一个 0 之前那段连续 1 的个数

        for i in 0..32 {
            if (num >> i) & 1 == 1 {
                // 遇到 1 
                curr += 1;
            } else {
                // 遇到 0
                // 把当前这个 0 翻转成 1，拼接左右两段
                ans = ans.max(prev + curr + 1);
                prev = curr;
                curr = 0;
            }
        }

        // 处理末尾没有 0 收尾的情况
        ans = ans.max(prev + curr + 1);
        ans
    }

    // 法二：滑动窗口
    // 思路：把最多翻转一个 0 看成窗口内最多允许一个 0
    pub fn reverse_bits(num: i32) -> i32 {
        let mut ans = 1;
        let mut left = 0;
        let mut zeros = 0;

        for right in 0..32 {
            // 窗口扩大，遇到 0 则增加计数
            if (num >> right) & 1 == 0 {
                zeros += 1;
            }

            // 当 0 的计数大于 1 时需要收缩窗口
            while zeros > 1 {
                if (num >> left) & 1 == 0 {
                    zeros -= 1;
                }
                left += 1;
            }

            // 此时窗口内最多只有 1 个 0，更新答案
            ans = ans.max(right - left + 1);
        }

        ans
    }
}