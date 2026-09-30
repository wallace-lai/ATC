struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 { return 0; }

        nums.sort_unstable();
        let mut left = 0_usize;
        let mut right = left + k as usize;
        let mut ans = i32::MAX;

        while right <= nums.len() {
            let dif = nums[right - 1] - nums[left];
            if dif < ans { ans = dif; }

            right += 1;
            left += 1;
        }

        ans
    }
}