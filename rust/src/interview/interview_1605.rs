struct Solution;

impl Solution {
    // 法一
    // 思路：n! 末尾的每个零都来自一个因子 10 = 2 x 5
    // 而 n! = 1 x 2 x 3 x ... x n里面，偶数远比 5 的倍数多，所以：
    // 问题等价于 n! 里有多少个因子 5
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            n /= 5;
            ans += n;
        }
        ans
    }
}