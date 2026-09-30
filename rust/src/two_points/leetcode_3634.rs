struct Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        // 要使移除的元素最少，则要使平衡子数组最大
        // 注意：你可以移除任意数量元素，这意味着你可以对数组进行排序
        nums.sort_unstable();
        let k = k as u64;
        let n = nums.len();
        let mut ans = n;
        let mut right = 0;
        
        for left in 0..n {
            while right < n && nums[left] as u64 * k >= nums[right] as u64 {
                right += 1;
            }

            ans = ans.min(n - right + left);
        }

        ans as i32
    }
}