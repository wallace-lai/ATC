struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        for k in (1..=arr.len()).step_by(2) {
            // println!("k is {k}");
            for w in arr.windows(k) {
                let tmp: i32 = w.iter().map(|num| *num).sum();
                ans += tmp;
            }
        }

        ans
    }
}