struct Solution;

impl Solution {
    pub fn max_digit_range(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n];
        let mut maxd = i32::MIN;

        for i in 0..n {
            let mut num = nums[i];
            let mut min = 9;
            let mut max = 0;
            while num > 0 {
                let r = num % 10;
                if r > max { max = r; }
                if r < min { min = r; }
                num /= 10;
            }
            d[i] = max - min;
            if d[i] > maxd { maxd = d[i]; }
        }

        let mut ans = 0;
        for i in 0..n {
            if d[i] == maxd {
                ans += nums[i];
            }
        }
        ans
    }
}