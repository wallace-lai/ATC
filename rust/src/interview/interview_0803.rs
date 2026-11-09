struct Solution;

impl Solution {
    #[cfg(any())]
    /// 前提：nums是严格递增的（题目不满足此条件）
    /// 思路：由于 nums 严格递增，设 f(i) = nums[i] - i
    ///     nums[i+1] >= nums[i] + 1
    ///     f(i+1) = nums[i+1]-(i+1) >= nums[i]+1-i-1 = f(i)
    /// 即 f(i) 单调非减，问题就变成了在单调序列 f(i)中，找到最小的 i 
    /// 使得 f(i) = 0，这天然适合二分
    /// 
    /// 做法：
    /// 1. 二分找到第一个 f(i) >= 0 的位置 p
    /// 2. 检查 f(p) == 0，若成立，则 p 就是最小的索引；否则索引不存在
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // 左闭右开区间
        let mut l = 0;
        let mut r = n;

        // 二分找到第一个 nums[i] >= i 的位置
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] < mid as i32 {
                l = mid + 1;
            } else {
                r = mid;    // r可能是答案，但还需要往左搜索
            }
        }

        // 检查该位置是否恰好相等
        if l < n && nums[l] as usize == l {
            l as i32
        } else {
            -1
        }
    }

    // 法一：假的二分法
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        
        // 返回 nums 中第一个满足条件的答案，-1 表示答案不存在
        fn helper(nums: &[i32], l: i32, r: i32) -> i32 {
            if l > r { return -1; }

            // 先递归搜索左半边的数组
            let mid = l + (r - l) / 2;
            let lans = helper(nums, l, mid - 1);
            if lans != -1 {
                return lans;
            } else if nums[mid as usize] == mid {
                return mid;
            }

            // 每找到答案再递归搜索右半边数组
            helper(nums, mid + 1, r)
        }

        helper(&nums, 0, nums.len() as i32 - 1)
    }
}