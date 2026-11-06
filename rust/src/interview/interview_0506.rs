struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：WA
    // 将 a 和 b 强制转换为 i64 后代码无法通过，根本原因
    // 在于 Rust 的 as 转换对负数执行了符号扩展，导致异或结果
    // 的高 32 位被“污染”，count_ones() 统计的就不再是纯粹的 32 位差异了。
    pub fn convert_integer(a: i32, b: i32) -> i32 {
        let a = a as i64;
        let b = b as i64;
        let c = a ^ b;
        c.count_ones() as i32
    }

    // 法二
    pub fn convert_integer(a: i32, b: i32) -> i32 {
        let c = a ^ b;
        c.count_ones() as i32
    }
}