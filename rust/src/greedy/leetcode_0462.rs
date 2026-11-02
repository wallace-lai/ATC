struct Solution;

impl Solution {
    // 法一：中位数贪心
    // 思路：要找到目标值 x 使得数轴上所有点到 x 的距离之和最小。让这个
    // 距离之和最小的点 x 就是数组的中位数。
    // 1. 对于奇数个元素，中位数就是排序后位于中间的那个数
    // 2. 对于偶数个元素，排序后中间有两个数，目标值 x 可以取这两个中位数
    // 之间（包含这两个数）的任意值
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let x = nums[n / 2];
        let mut ans = 0;

        for i in 0..n {
            ans += nums[i].abs_diff(x);
        }

        ans as i32
    }
}