struct Solution;

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();
        let n = arr.len();
        let slice = &arr[n / 20..(19 * n / 20)];
        let sum: f64 = slice.iter().map(|f| *f as f64).sum();

        sum / n as f64 * 0.9
    }
}