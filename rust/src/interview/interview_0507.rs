struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：WA
    // 0xAAAAAAAA 这个字面量的数值超出了 i32 的表示范围
    pub fn exchange_bits(num: i32) -> i32 {
        ((num & 0x55555555) << 1) | ((num & 0xAAAAAAAA) >> 1)
    }

    // 法二
    pub fn exchange_bits(num: i32) -> i32 {
        let n = num as u32;
        let even = n & 0x5555_5555;
        let odd = n & 0xAAAA_AAAA;
        ((even << 1) | (odd >> 1)) as i32
    }
}