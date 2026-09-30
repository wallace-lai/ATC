struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let b = s.as_bytes();
        let n = s.len();

        let len = if n % k == 0 { n / k } else { n / k + 1 };
        let mut ans = Vec::with_capacity(len);

        for i in 0..len {
            let mut group: Vec<u8> = Vec::with_capacity(k);
            let end = std::cmp::min(n, (i + 1) * k);
            group.extend_from_slice(&b[i * k..end]);
            ans.push(unsafe { String::from_utf8_unchecked(group) });
        }
        if len == n / k + 1 {
            let repeat = k - ans[len - 1].len();
            for _ in 0..repeat {
                ans[len - 1].push(fill);
            }
        }

        ans
    }
}