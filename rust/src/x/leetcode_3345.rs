struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn check(mut n: i32, t: i32) -> bool {
        let mut prod = 1;
        while n > 0 {
            prod *= n % 10;
            n /= 10;
            if prod == 0 {
                break;
            }
        }

        prod % t == 0
    } 

    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        while !Self::check(n, t) {
            n += 1;
        }

        n
    }
}