struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        fn bit_sum(mut n: i32) -> i32 {
            let mut ans = 0;
            while n > 0 {
                ans += n % 10;
                n /= 10;
            }
            ans
        }

        let sum1: i32 = nums.iter().sum();
        let mut sum2 = 0;
        for num in nums {
            sum2 += bit_sum(num);
        }

        (sum1 - sum2).abs()
    }
}