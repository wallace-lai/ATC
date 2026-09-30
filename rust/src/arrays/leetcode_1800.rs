struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        let mut sum = nums[0];

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                ans = ans.max(sum);
                sum = nums[i];
                continue;
            }

            sum += nums[i];
        }
        ans = ans.max(sum);

        ans
    }
}