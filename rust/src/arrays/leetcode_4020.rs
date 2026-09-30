struct Solution;

impl Solution {
    pub fn elevator_requests(_: i32, requests: Vec<i32>) -> i32 {
        let mut ans = requests[0];
        for i in 1..requests.len() {
            ans += (requests[i] - requests[i - 1]).abs();
        }
        ans
    }
}