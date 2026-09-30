use std::i32;

struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        for num in nums {
            let tmp = {
                let mut sum = 0;
                let mut n = num;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                }
                sum
            };

            if tmp < ans { ans = tmp; }
        }
        ans
    }
}