struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut v1 = Vec::with_capacity(n as usize);
        for i in 1..=(n / 2) {
            v1.push(i);
        }

        let v2 = Vec::from_iter(
            v1.iter().map(|num| 0 - *num)
        );

        if n & 1 == 1 { v1.push(0); }
        v1.extend(v2.into_iter());

        v1
    }
}