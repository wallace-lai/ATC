struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut ans = 0;

        fn helper(mut n: i32) -> bool {
            let mut flag = 0;
            while n > 0 {
                flag ^= if n & 1 == 1 { 1 } else { 0 };
                n /= 10;
            }
            if flag & 1 == 1 { false } else { true }
        }

        for i in 1..=num {
            if helper(i) {
                ans += 1;
            }
        }

        ans
    }
}