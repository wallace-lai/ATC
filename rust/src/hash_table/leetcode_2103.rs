struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn count_points(rings: String) -> i32 {
        let two_n = rings.len();
        let mut ans = 0;
        let mut count = [0u32; 10];

        let slice = rings.as_bytes();
        for i in (0..two_n).step_by(2) {
            let idx = (slice[i + 1] - b'0') as usize;
            match slice[i] {
                b'R' => count[idx] |= 1 << 2,
                b'G' => count[idx] |= 1 << 1,
                b'B' => count[idx] |= 1,
                _ => ()
            };
        }

        for i in 0..count.len() {
            if count[i] == 7 {
                ans += 1;
            }
        }

        ans
    }
}