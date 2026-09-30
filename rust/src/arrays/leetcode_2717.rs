struct Solution;

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let mut p1 = 0;
        let mut pn = 0;
        for i in 0..len {
            if nums[i as usize] == 1 {
                p1 = i;
            } else if nums[i as usize] == len {
                pn = i;
            }
        }

        if pn < p1 {
            p1 + len - 2 - pn
        } else {
            p1 + len - 1 - pn
        }
    }
}