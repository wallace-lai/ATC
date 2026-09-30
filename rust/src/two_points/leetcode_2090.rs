struct Solution;

impl Solution {
    // 0ms
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = k as usize * 2 + 1;
        let mut sum = 0_usize;
        let mut left = 0;
        let mut ans = vec![-1; nums.len()];

        for right in 0..nums.len() {
            sum += nums[right] as usize;
            if right + 1 < len { continue; }

            ans[left + len / 2] = (sum / len) as i32;

            sum -= nums[left] as usize;
            left += 1;
        }

        ans
    }
}