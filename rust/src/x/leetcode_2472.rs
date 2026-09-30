struct Solution;

impl Solution {
    pub fn check(s: &[u8], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }

    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let k = k as usize;

        let mut ans = 0;
        let mut start = 0;

        for r in k - 1..n {
            let mut l = r + 1 - k;
            if l >= start && Self::check(b, l, r) {
                ans += 1;
                start = r + 1;
                continue;
            }

            if r >= k {
                l = r - k;
                if l >= start && Self::check(b, l, r) {
                    ans += 1;
                    start = r + 1;
                }
            }
        }

        ans
    }
}