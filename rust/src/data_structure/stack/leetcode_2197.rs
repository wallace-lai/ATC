struct Solution;

impl Solution {
    // 4ms，击败100%
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let mut v: Vec<i32> = Vec::with_capacity(nums.len());

        for mut num in nums {
            while let Some(&top) = v.last() {
                let g = gcd(top, num);
                if g == 1 { break; }
                num = top / g * num;
                v.pop();
            }
            v.push(num);
        }

        v
    }
}
