struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut len = 0;
        for num in nums {
            if num & 1 == 0 && num % 3 == 0 {
                sum += num;
                len += 1;
            }
        }

        if sum == 0 { 0 } else { sum / len }
    }
}