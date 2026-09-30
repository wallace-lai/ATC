struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let b = blocks.as_bytes();
        let n = b.len();
        let k = k as usize;

        let mut ans = k as i32;
        let mut white = 0;

        for right in 0..n {
            // 扩展窗口
            if b[right] == b'W' { white += 1; }
            // 窗口未满k，继续扩展
            if right + 1 < k { continue; }

            // 窗口大小刚好为k
            ans = std::cmp::min(ans, white);

            // 收缩窗口
            if b[right + 1 - k] == b'W' { white -= 1; }
        }

        ans
    }
}
