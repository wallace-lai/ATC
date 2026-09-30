struct Solution;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;

        while n > 0 {
            let x = n % 10;
            if x > max1 {
                max2 = max1;
                max1 = x;
            } else if x > max2 {
                max2 = x;
            }

            n /= 10;
        }

        max1 * max2
    }
}