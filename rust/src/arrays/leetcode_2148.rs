struct Solution;

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let &min = nums.iter().min().unwrap();
        let &max = nums.iter().max().unwrap();
        let mut ans = 0;
        for num in nums {
            if min < num && num < max {
                ans += 1;
            }
        }
        ans
    }
}