struct Solution;

impl Solution {
    // 0ms
    // pub fn balanced_string_split(s: String) -> i32 {
    //     let mut count = [0, 0];
    //     let mut ans = 0;

    //     for i in 0..s.len() {
    //         if s.as_bytes()[i] == b'L' {
    //             count[0] += 1;
    //         } else {
    //             count[1] += 1;
    //         }

    //         if count[0] == count[1] && count[0] != 0 {
    //             ans += 1;
    //             count.fill(0);
    //         }
    //     }

    //     ans
    // }

    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut ans = 0;

        for i in 0..s.len() {
            count += if s.as_bytes()[i] == b'L' { 1 } else { -1 };
            if count == 0 { ans += 1; }
        }

        ans
    }
}