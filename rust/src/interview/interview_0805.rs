struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：迭代
    pub fn multiply(a: i32, b: i32) -> i32 {
        let mut ans = 0;
        for _ in 0..b {
            ans += a;
        }
        ans
    }

    #[cfg(any())]
    // 法二：递归
    pub fn multiply(a: i32, b: i32) -> i32 {
        if b == 1 { return a; }
        a + Self::multiply(a, b - 1)
    }

    #[cfg(any())]
    // 法三：递归二分（快速幂）
    // 思路：A * B = A * (B / 2) + A * (B / 2)
    // 如果 B 是奇数，再额外加上一个 A 即可，这样每次递归 B 都会减半
    pub fn multiply(a: i32, b: i32) -> i32 {
        if b == 0 { return 0; }

        let half = Self::multiply(a, b / 2);
        if b & 1 == 1 {
            half + half + a
        } else {
            half + half
        }
    }

    #[cfg(any())]
    // 法四：二进制乘法器
    pub fn multiply(mut a: i32, mut b: i32) -> i32 {
        // 优化：让较小的数作为乘数，减少循环次数
        if a < b { std::mem::swap(&mut a, &mut b); }

        let mut ans = 0;
        while b > 0 {
            // 检查 B 的最低位是否为 1
            if b & 1 == 1 { ans += a; }
            a <<= 1;    // A 左移一位，相当于乘以2
            b >>= 1;    // B 右移一位，处理下一位
        }

        ans
    }

    // 法五：俄罗斯农民乘法（本质和法四一样）
    // 思路：A * B = (A * 2) * (B / 2)，如果 B 是奇数，则先提出一个 A，即：
    // A * B = A + (A * 2) * (B / 2)，不断重复，直到 B 变为1
    pub fn multiply(a: i32, b: i32) -> i32 {
        if b == 0 { return 0; }

        if b & 1 == 1 {
            a + Self::multiply(a << 1, b >> 1)
        } else {
            Self::multiply(a << 1, b >> 1)
        }
    }
}