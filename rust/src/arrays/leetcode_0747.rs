struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        use std::mem::swap;

        let mut max1 = nums[0];
        let mut max2 = nums[1];
        let mut max1_idx = 0;
        let mut max2_idx = 1;

        if max2 > max1 {
            swap(&mut max1, &mut max2);
            swap(&mut max1_idx, &mut max2_idx);
        }

        for i in 2..nums.len() {
            if nums[i] > max1 {
                max2 = max1;
                max1 = nums[i];
                max1_idx = i;
            } else if nums[i] > max2 {
                max2 = nums[i];
            }
        }

        if max1 >= max2 * 2 { max1_idx as i32 } else { -1 }
    }
}