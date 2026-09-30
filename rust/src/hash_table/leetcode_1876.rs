struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let len = s.len();
        if len < 3 { return 0; }

        let mut ans = 0;
        for substr in s.as_bytes().windows(3) {
            if substr[0] != substr[1] &&
                substr[1] != substr[2] &&
                substr[2] != substr[0] {
                ans += 1;
            }
        }

        ans
    }
}