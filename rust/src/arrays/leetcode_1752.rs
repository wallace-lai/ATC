struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut source = nums.clone();
        source.sort_unstable();

        let n = source.len();
        for _ in 0..n {
            source.rotate_left(1);
            if nums == source {
                return true;
            }
        }

        false
    }
}