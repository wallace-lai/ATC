struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：暴力枚举，超时
    pub fn number_of2s_in_range(n: i32) -> i32 {
        let mut count = 0;
        for i in 0..=n {
            let mut x = i;
            while x > 0 {
                if x % 10 == 2 {
                    count += 1;
                }
                x /= 10;
            }
        }
        count
    }

    // 法二：逐位统计法
    // 思路：按位来数，固定一个位（个位、十位、百位...），统计这个位是2的个数有多少
    // 具体：
    // 假设正在统计第 k 位，权重 factor = 10^k，把 n 拆分成三部分
    // n = high | cur | low
    // cur = (n / factor) % 10 - 当前数字
    // high = n / (factor * 10) - 当前数字左边高位
    // low = n % factor - 当前数字右边低位
    //
    // 三种情况
    // cur > 2 - 则该位出现2的次数为： (high + 1) * factor
    // cur == 2 - 该位出现2的次数为：high * factor + low + 1
    // cur < 2 - 该位出现2的次数为：high * factor
    pub fn number_of2s_in_range(n: i32) -> i32 {
        if n < 2 { return 0; }

        let n = n as u64;
        let mut count: u64 = 0;
        let mut factor: u64 = 1;    // 从个位开始统计

        while factor <= n {
            let high = n / (factor * 10);
            let curr = (n / factor) % 10;
            let low = n % factor;

            count += match curr {
                c if c > 2 => (high + 1) * factor,
                c if c == 2 => high * factor + low + 1,
                _ => high * factor,
            };

            factor *= 10;
        }

        count as i32
    }
}