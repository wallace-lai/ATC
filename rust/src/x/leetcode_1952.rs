struct Solution;

impl Solution {
    pub fn is_three(n: i32) -> bool {

        fn is_prime(n: i32) -> bool {
            let n = n as i64;
            let mut i: i64 = 2;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 1;
            }
            n >= 2
        }

        let s = n.isqrt();
        if s * s != n || n < 4 {
            return false;
        }

        // 除开 1 和 n 本身，n有且仅能有一个质因数
        is_prime(s)
    }
}