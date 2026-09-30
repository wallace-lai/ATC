struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max = -1;
        let mut ans = vec![0, 0, 0];
        for w in num.as_bytes().windows(3) {
            if w[0] == w[1] && w[1] == w[2] {
                let tmp = (w[0] - b'0') as i32 * 100 +
                    (w[1] - b'0') as i32 * 10 +
                    (w[2] - b'0') as i32;
                if tmp > max {
                    max = tmp;
                    ans[0] = w[0];
                    ans[1] = w[1];
                    ans[2] = w[2];
                }
            }
        }

        if max > -1 {
            return unsafe { String::from_utf8_unchecked(ans) };
        }

        "".to_string()
    }
}