struct Solution;

impl Solution {
    pub fn max_pair_strength(nums: Vec<i32>) -> i64 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let mut ans = 0;
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let a = nums[i];
                let b = nums[j];
                let g = gcd(a, b) as i64;
                let tmp = a as i64 / g * b as i64 / g;
                if tmp > ans { ans = tmp; }
            }
        }

        ans as i64
    }
}