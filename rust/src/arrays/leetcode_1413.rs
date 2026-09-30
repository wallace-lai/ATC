struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut start = 1;

        fn check(nums: &Vec<i32>, mut start: i32) -> bool {
            for &num in nums.iter() {
                if start + num < 1 {
                    return false;
                }
                start += num;
            }
            true
        }

        while !check(&nums, start) {
            start += 1;
        }

        start
    }
}