struct Solution;

impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut v = Vec::with_capacity(16);
        while num > 0 {
            v.push(num % 10);
            num /= 10;
        }
        v.reverse();

        for i in 0..v.len() {
            let mut p: i32 = -1;
            for j in (i + 1)..v.len() {
                if (v[j] ^ v[i]) & 1 == 0 &&
                    v[j] > v[i] && 
                    (p == -1 || v[j] > v[p as usize]) {
                    p = j as i32;
                }
            }
            if p != -1 {
                v.swap(i, p as usize);
            }
        }

        let mut ans = 0;
        for i in 0..v.len() {
            ans = ans * 10 + v[i];
        }
        ans
    }
}