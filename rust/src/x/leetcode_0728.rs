struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        
        fn check(a: i32) -> bool {
            let mut n = a;
            while n > 0 {
                let r = n % 10;
                if r == 0 || a % r != 0 {
                    return false;
                }
                n /= 10;
            }
            true
        }

        let mut v = Vec::new();
        for i in left..=right {
            if check(i) {
                v.push(i);
            }
        }
        v
    }
}