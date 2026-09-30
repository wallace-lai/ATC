struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let &min = nums.iter().min().unwrap();
        let &max = nums.iter().max().unwrap();
        gcd(max, min)
    }
}