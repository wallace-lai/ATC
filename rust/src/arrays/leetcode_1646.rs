struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let n = n as usize;
        if n == 0 { return 0; }
        let mut v = vec![0; n + 1];

        v[0] = 0;
        v[1] = 1;
        let mut ans = 1;

        for i in 0..(n + 1) {
            if 2 <= 2 * i && 2 * i <= n {
                v[2 * i] = v[i];
                if v[i] > ans { ans = v[i]; }
            }
            if 2 <= 2 * i + 1 && 2 * i + 1 <= n {
                v[2 * i + 1] = v[i] + v[i + 1];
                if v[i] + v[i + 1] > ans {
                    ans = v[i] + v[i + 1];
                }
            }
        }

        ans
    }
}