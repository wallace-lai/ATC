struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let n = s.len();
        let b = s.as_bytes();

        let mut ans = 1;
        let mut prev_chr = b[0];
        let mut prev_idx = 0;
        let mut curr_idx = 1;

        while curr_idx < n {
            if b[curr_idx] != prev_chr {
                ans = std::cmp::max(ans, curr_idx - prev_idx);
                prev_idx = curr_idx;
                prev_chr = b[prev_idx];
            }

            curr_idx += 1;
        }
        ans = std::cmp::max(ans, curr_idx - prev_idx);

        ans as i32
    }
}