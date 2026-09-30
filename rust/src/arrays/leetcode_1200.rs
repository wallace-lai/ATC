struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min_abs = i32::MAX;
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] < min_abs {
                min_abs = arr[i] - arr[i - 1];
            }
        }

        let mut ans = Vec::new();
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min_abs {
                ans.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ans
    }
}