struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let k = k as usize;
        let s = num.to_string();
        let b = s.as_bytes();

        let mut ans = 0;
        for slice in b.windows(k) {
            let substr: i32 = slice.iter()
                .map(|&c| (c - b'0') as i32)
                .fold(0, |acc, x| acc * 10 + x);
            if substr != 0 && num % substr == 0 { ans += 1; }
        }
        ans
    }
}