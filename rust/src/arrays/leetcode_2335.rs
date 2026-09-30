struct Solution;

impl Solution {
    pub fn fill_cups(mut a: Vec<i32>) -> i32 {
        let mut ans = 0;

        while a[0] > 0 && a[1] > 0 && a[2] > 0 {
            let (mut first, mut second) =
                if a[0] > a[1] {
                    (0, 1)
                } else {
                    (1, 0)
                };
            if a[2] > a[first] {
                second = first;
                first = 2;
            } else if a[2] > a[second] {
                second = 2;
            }

            a[first] -= 1;
            a[second] -= 1;
            ans += 1;
        }

        let v: Vec<i32> = a.iter()
            .filter(|n| **n > 0)
            .map(|n| *n)
            .collect();

        match v.len() {
            1 => {
                ans += v[0];
            },
            2 => {
                let min = std::cmp::min(v[0], v[1]);
                ans += v[0] + v[1] - min;
            },
            _ => {}
        }

        ans
    }
}