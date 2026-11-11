struct Solution;

impl Solution {
    pub fn count_operations(mut n1: i32, mut n2: i32) -> i32 {
        let mut ans = 0;

        loop {
            if n1 == 0 || n2 == 0 {
                break;
            }

            let min = std::cmp::min(n1, n2);
            if n1 == min { n2 -= min; ans += 1; }
            if n2 == min { n1 -= min; ans += 1; }
        }

        ans
    }
}