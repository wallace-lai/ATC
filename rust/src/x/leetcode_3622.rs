struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut sum = 0;
        let mut prod = 1;
        let mut num = n;
        while num > 0 {
            let t = num % 10;
            sum += t;
            prod *= t;

            num /= 10;
        }

        n % (sum + prod) == 0
    }
}