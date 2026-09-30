struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn limit_occurrences(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut cnt = 0;
        let mut idx = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i - 1] == nums[i] {
                cnt += 1;
            } else {
                cnt = 0;
            }

            if cnt < k {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        nums.resize(idx as usize, 0);
        nums
    }
}