struct Solution;

impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        let n = s.len();
        let bstr = s.as_bytes();
        for i in 0..=(n / 2) {
            if bstr[i] == bstr[n - i - 1] {
                return i as i32;
            }
        }

        -1
    }
}