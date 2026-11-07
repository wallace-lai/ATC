struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 { return 1; }
        let n = n as u32;
        // 通过 leading_zeros() 获取数字的有效位数
        let bits = u32::BITS - n.leading_zeros();
        let mask = if bits == 32 { u32::MAX } else { (1u32 << bits) - 1 };
        (n ^ mask) as i32
    }
}