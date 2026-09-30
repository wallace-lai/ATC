struct Solution;

impl Solution {
    // 0ms - 击败100%
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ans = 0;

        for &num in nums.iter() {
            sum += num;
            if sum == 0 {
                ans += 1;
            }
        }

        ans
    }
}