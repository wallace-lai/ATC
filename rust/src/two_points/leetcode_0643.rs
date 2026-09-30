struct Solution;

impl Solution {
    // 0ms
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = 0.0;
        let mut ans = f64::MIN;

        for right in 0..nums.len() {
            sum += nums[right] as f64;
            if right + 1 < k { continue; }

            let avg = sum / k as f64;
            if avg > ans { ans = avg; }

            sum -= nums[right + 1 - k] as f64;
        }

        ans
    }
}
