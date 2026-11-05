struct Solution;

impl Solution {
    pub fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
        // 计算区间长度
        let len = (j - i + 1) as u32;

        // 构造目标区间的掩码，得到 [i, j] 区间全为1，其余全为 0 的掩码
        // 用 i64 构造掩码，避免 len == 32 时 1_i32 << 32 溢出
        let mask = (((1_i64 << len) - 1) << i) as i32;

        // 先清空 N 的 [i, j] 区间，再填入 M << i
        (n & !mask) | (m << i)
    }
}