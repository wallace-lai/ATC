struct Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let b = word.as_bytes();
        let n = b.len();

        let mut ans = 0;
        let mut prev: i32 = b'a' as i32;
        for i in 0..n {
            let curr: i32 = b[i] as i32;
            if curr != prev {
                // 移动 + 键入
                let d1 = if curr - prev > 0 { curr - prev } else { curr - prev + 26 };
                let d2 = if prev - curr > 0 { prev - curr } else { prev - curr + 26 };
                ans += d1.min(d2) + 1;

                prev = curr;
            } else {
                // 键入
                ans += 1;
            }
        }

        ans
    }
}
