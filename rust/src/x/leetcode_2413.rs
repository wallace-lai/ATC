struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        n / gcd(2, n) * 2
    }
}