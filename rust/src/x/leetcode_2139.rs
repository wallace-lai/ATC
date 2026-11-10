struct Solution;

impl Solution {
    pub fn min_moves(mut target: i32, mut max_doubles: i32) -> i32 {
        let mut ans = 0;

        loop {
            if target == 1 || max_doubles == 0 {
                break;
            }

            if target & 1 == 1 {
                target -= 1;
                ans += 1;
            } else {
                target /= 2;
                ans += 1;
                max_doubles -= 1;
            }
        }

        while target > 1 {
            ans += 1;
            target -= 1;
        }

        ans
    }
}