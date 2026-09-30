struct Solution;

impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = f64::MAX;

        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            let average = (nums[left] as f64 + nums[right] as f64) / 2.0;
            if average < ans { ans = average; }
            left += 1;
            right -= 1;
        }

        ans
    }
}