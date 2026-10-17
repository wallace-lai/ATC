struct Solution;

impl Solution {
    // 法一：打表
    // pub fn is_power_of_four(n: i32) -> bool {
    //     if n < 1 { return false; }
    //     let n = n as u32;
    //     match n {
    //         1 | 4 | 16 | 64 | 256 | 1024 | 4096 | 16384 |
    //         65536 | 262144 | 1048576 | 4194304 | 16777216 |
    //         67108864 | 268435456 | 1073741824 => { return true; },
    //         _ => { return false; }
    //     }
    // }

    // 法二：递归
    pub fn is_power_of_four(n: i32) -> bool {
        if n < 1 { return false; }
        if n == 1 { return true; }
        if n % 4 != 0 { return false; }
        Self::is_power_of_four(n / 4)
    }
}