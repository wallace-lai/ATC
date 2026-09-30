struct Solution;

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut ans = 0_i64;
        let n = nums.len() as i32;
        let mut left = 0;
        let mut right = n - 1;

        pub fn concat(a: i64, b: i64) -> i64 {
            let mut len = 0;
            let mut n = b;
            while n > 0 {
                len += 1;
                n /= 10;
            }

            a * 10_i64.pow(len) + b
        }

        while left <= right {
            if left == right {
                ans += nums[left as usize] as i64;
            } else {
                ans += concat(nums[left as usize] as i64, nums[right as usize] as i64);
            }

            left += 1;
            right -= 1;
        }

        ans
    }
}
