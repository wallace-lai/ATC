struct Solution;

impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable_by_key(|&n| if n >= 0 { n } else { -n });
        let mut n = nums.len();
        let mut ans = 0i64;
    
        if n & 1 == 1 {
            ans += nums[n - 1] as i64 * nums[n - 1] as i64;
            n -= 1;
        }
        if n == 0 { return ans; }

        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            let d = nums[right] * nums[right] - nums[left] * nums[left];
            ans += d as i64;
            left += 1;
            right -= 1;
        }

        ans
    }
}