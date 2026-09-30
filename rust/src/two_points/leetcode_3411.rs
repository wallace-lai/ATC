struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        for d in (2..=n).rev() {
            for w in nums.windows(d) {
                let mut p = 1;
                let mut l = 1;
                let mut g = 0;
                for &x in w {
                    p *= x;
                    g = gcd(g, x);
                    l = l / gcd(l, x) * x; 
                }
                if p == g * l { return d as i32; }
            }
        }
        
        unreachable!()
    }
}