struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        // pre[i] = sum(nums[0..=i])
        // suf[i] = sum(nums[i..])
        let n = nums.len();
        let mut pre = vec![0; n];
        let mut suf = vec![0; n];
        for i in 0..n {
            pre[i] = nums[i] + if i == 0 { 0 } else { pre[i - 1] };
        }
        for i in (0..=(n - 1)).rev() {
            suf[i] = nums[i] + if i == n - 1 { 0 } else { suf[i + 1] };
        }

        let mut ans = 0;
        for i in 0..n {
            if nums[i] == 0 {
                if pre[i] == suf[i] + 1 ||
                    pre[i] + 1 == suf[i] {
                    ans += 1; 
                } else if pre[i] == suf[i] {
                    ans += 2;
                }
            }
        }

        ans
    }
}