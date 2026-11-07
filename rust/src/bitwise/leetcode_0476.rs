struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let n = num as u32;
        // 通过 leading_zeros() 获取数字的有效位数
        let bits = u32::BITS - n.leading_zeros();
        let mask = if bits == 32 { u32::MAX } else { (1u32 << bits) - 1 };
        (n ^ mask) as i32
    }
}