struct Solution;

impl Solution {
    // WA
    // pub fn min_add_to_make_valid(s: String) -> i32 {
    //     let b = s.as_bytes();
    //     let n = b.len();
    //     let mut c: i32 = 0;

    //     for i in 0..n {
    //         let d = if b[i] == b'(' { 1 } else { -1 };
    //         c += d;
    //     }

    //     c.abs()
    // }

    pub fn min_add_to_make_valid(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut v = Vec::with_capacity(n);

        for i in 0..n {
            if b[i] == b')' && v.len() > 0 &&
                v.last().unwrap() == &b'(' {
                v.pop();
                continue;
            }

            v.push(b[i]);
        }

        v.len() as i32
    }
}