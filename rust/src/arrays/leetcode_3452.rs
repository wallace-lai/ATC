struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;

        let k = k as usize;
        let len = nums.len();
        for i in 0..nums.len() {
            let check1 = if i < k { true } else { nums[i] > nums[i - k] };
            let check2 = if i >= len - k { true } else { nums[i] > nums[i + k] };
            if check1 && check2 {
                ans += nums[i];
            }
        }

        ans
    }
}