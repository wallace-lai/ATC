struct Solution;

impl Solution {
    pub fn total_money(mut n: i32) -> i32 {
        let week_days = 1 + 2 + 3 + 4 + 5 + 6 + 7;
        let mut i = 0;
        let mut ans = 0;

        while n >= 7 {
            n -= 7;
            ans += week_days + i * 7;
            i += 1;
        }

        i += 1;
        while n > 0 {
            n -= 1;
            ans += i;
            i += 1;
        }

        // println!("n is {n}, i is {i}");

        ans
    }
}