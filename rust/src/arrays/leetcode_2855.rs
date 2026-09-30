struct Solution;

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min = 0;
        for i in 0..n {
            if nums[i] < nums[min] {
                min = i;
            }
        }

        for d in 0..(n - 1) {
            let i1 = (min + d) % n;
            let i2 = (min + d + 1) % n;
            if nums[i1] > nums[i2] {
                return -1;
            }
        }

        if min == 0 { 0 } else { (n - min) as i32 }
    }
}