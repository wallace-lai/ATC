struct Solution;

impl Solution {
    // 法一：手动实现
    // pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    //     let mut n = nums.len() as u32;
    //     if n < 3 {
    //         return n as i32;
    //     }

    //     n |= n >> 1;
    //     n |= n >> 2;
    //     n |= n >> 4;
    //     n |= n >> 8;
    //     n |= n >> 16;
        
    //     (n + 1) as i32
    // }

    /// 下面这些方法定义在Rust的原生整数类型（u32 / u64）上面
    /// next_power_of_two() - 返回大于或等于n的最小2的幂
    /// checked_next_power_of_two() - 功能相同，但结果溢出时返回None而非panic
    /// is_power_of_two() - 检查一个数是否为2的幂
    /// 
    /// leading_zeros() - 返回二进制中最高位1之前连续的0的个数
    /// trailing_zeros() - 返回二进制中最低位1之后0的个数
    /// 
    /// count_ones() - 返回二进制中1的个数（汉明重量）
    /// count_zeros() - 返回二进制中0的个数
    /// 
    /// rotate_left() / rotate_right() - 循环左移或右移，移出的位会补到另一端
    /// 
    /// swap_bytes() - 交换字节顺序
    /// reverse_bits() - 反转所有的位
    /// 
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;
        if n < 3 {
            return n as i32;
        }

        (n + 1).next_power_of_two() as i32
    }
}