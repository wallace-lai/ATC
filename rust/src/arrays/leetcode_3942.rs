struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return 0; }

        let mut d = nums[1] - nums[0];
        if d.abs() != 1 {
            d = nums[2] - nums[1];
            if d.abs() != 1 {
                return -1;
            }
        }

        let mut p = 0;
        for i in 1..n {
            if nums[i] - nums[i - 1] != d {
                if p > 0 {
                    return -1;
                }
                p = i;
            }
        }
        
        if d == 1 {
            return std::cmp::min(p as i32, n as i32 - p as i32 + 2);
        }

        std::cmp::min(p as i32, n as i32 - p as i32) + 1
    }
}