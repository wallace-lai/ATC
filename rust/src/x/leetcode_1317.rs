struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {

        fn check(mut a: i32) -> bool {
            while a > 0 {
                if a % 10 == 0 {
                    return false;
                }
                a /= 10;
            }
            true
        }

        let m = n / 2;
        for a in 1..=m {
            if check(a) && check(n - a) {
                return vec![a, n - a];
            }
        }

        // 题目保证至少有一个答案存在
        unreachable!()
    }
}