struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut v = Vec::with_capacity(strs.len());
        for s in &strs {
            if !s.as_bytes().iter().all(|c| c.is_ascii_digit()) {
                v.push(s.len() as i32);
                continue;
            }

            let mut num = 0;
            for &c in s.as_bytes() {
                num = num * 10 + (c - b'0') as i32;
            }
            v.push(num);
        }

        let ans = v.iter().max().unwrap();
        *ans
    }
}