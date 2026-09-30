struct Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let n = nums.len();

        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if nums[i] < nums[j] && nums[k] < nums[j] {
                        let sum = nums[i] + nums[j] + nums[k];
                        if sum < ans { ans = sum; }
                    }
                }
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}