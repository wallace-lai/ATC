struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let k = arr.len() / 4 + 1;
        for w in arr.windows(k) {
            if w.windows(2).all(|p| p[0] == p[1]) {
                return w[0];
            }
        }

        unreachable!()
    }
}