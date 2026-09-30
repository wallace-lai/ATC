struct Solution;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort_unstable();

        let mut i = 0;
        while i < len && nums[i] == 0 {
            i += 1;
        }
        if i == len { return 0; }

        let mut ans = 1;
        for k in (i + 1)..len {
            if nums[k] > nums[k - 1] {
                ans += 1;
            }
        }
        ans
    }
}