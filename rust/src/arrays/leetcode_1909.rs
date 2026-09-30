struct Solution;

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let check = |idx| -> bool {
            for i in 1..(nums.len() - 1) {
                let mut prev = i - 1;
                if prev >= idx { prev += 1; }
                let mut curr = i;
                if curr >= idx { curr += 1; }
                if nums[curr] <= nums[prev] {
                    return false;
                }
            }
            true
        };

        let n = nums.len();
        for i in 1..n {
            if nums[i - 1] >= nums[i] {
                return check(i - 1) || check(i);
            }
        }

        true
    }
}