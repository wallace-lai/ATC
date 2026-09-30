struct Solution;

impl Solution {
    pub fn score_balance(s: String) -> bool {
        let n = s.len();
        let mut pre = vec![0; n];

        pre[0] = (s.as_bytes()[0] - b'a' + 1) as i32;
        for i in 1..n {
            pre[i] = pre[i - 1] + 
                (s.as_bytes()[i] - b'a' + 1) as i32;
        }

        for i in 0..(n - 1) {
            if pre[i] == pre[n - 1] - pre[i] {
                return true;
            }
        }

        false
    }
}