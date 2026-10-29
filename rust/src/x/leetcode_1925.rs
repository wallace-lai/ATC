struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;

        for a in 1..n {
            for b in 1..n {
                let tmp = a * a + b * b;
                if tmp > n * n { continue; }

                let mut left = b + 1;
                let mut right = n;
                while left <= right {
                    let c = left + (right - left) / 2;
                    let square = c * c;
                    if square == tmp {
                        ans += 1;
                        break;
                    } else if square > tmp {
                        right = c - 1;
                    } else {
                        left = c + 1;
                    }
                }
            }
        }

        ans
    }
}
