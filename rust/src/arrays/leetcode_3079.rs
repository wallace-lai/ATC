struct Solution;

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for num in nums {
            let (len, max) = {
                let mut n = num;
                let mut len = 0;
                let mut max = 0;
                while n > 0 {
                    let r = n % 10;
                    if r > max { max = r; }
                    n /= 10;
                    len += 1;
                }
                (len, max)
            };

            ans += max * match len {
                1 => 1,
                2 => 11,
                3 => 111,
                4 => 1111,
                _ => 0
            };
        }

        ans
    }
}