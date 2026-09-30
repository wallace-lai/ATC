struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for num in nums {
            let r = num % 3;
            match r {
                1 | 2 => { ans += 1; }
                _ => {}

            };
        }
        ans
    }
}