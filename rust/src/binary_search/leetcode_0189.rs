struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        if k == 0 || n < 2 { return; }

        let slice = nums.as_mut_slice();
        slice.reverse();
        &slice[0..k].reverse();
        &slice[k..n].reverse();
    }
}