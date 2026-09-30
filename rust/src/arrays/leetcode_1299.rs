struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut suf = vec![-1; arr.len()];
        for i in (0..(arr.len() - 1)).rev() {
            suf[i] = std::cmp::max(arr[i + 1], suf[i + 1]);
        }

        suf
    }
}