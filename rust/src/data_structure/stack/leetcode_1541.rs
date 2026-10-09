struct Solution;

impl Solution {
    // 法一：从左往右配对
    // 3ms，击败40%
    // pub fn min_insertions(s: String) -> i32 {
    //     let b = s.as_bytes();
    //     let n = b.len();
    //     let mut i = 0;
    //     let mut ans = 0;
    //     let mut left = 0;

    //     while i < n {
    //         if b[i] == b'(' {
    //             left += 1;
    //             i += 1;
    //             continue;
    //         }

    //         // 遇到右括号
    //         if left > 0 {
    //             left -= 1;  // 一个左括号已配对
    //         } else {
    //             ans += 1;   // 当前的右括号没有配对的左括号，需要插入一个
    //         }

    //         // 必须有两个连续的右括号
    //         if i + 1 < n && b[i + 1] == b')' {
    //             i += 2;     // 刚好有两个连续的右括号，跳过即可
    //         } else {
    //             i += 1;
    //             ans += 1;   // 否则必须插入一个右括号
    //         }
    //     }

    //     ans + left * 2      // 为剩下未配对的左括号插入2倍数量的右括号
    // }

    pub fn min_insertions(s: String) -> i32 {
        let mut left_need = 0;
        let mut right_need = 0;

        for c in s.chars() {
            if c == '(' {   // 遇到左括号，需要2个右括号来配对
                // 如果当前需要的右括号是奇数，说明之前有未配对的单个右括号
                if right_need & 1 == 1 {
                    left_need += 1;
                    right_need -= 1;
                }
                right_need += 2;
            } else {        // 遇到右括号
                if right_need == 0 {
                    // 没有待匹配的左括号，需要插入一个左括号
                    left_need += 1;
                    right_need += 1;
                } else {
                    // 匹配到了一个右括号
                    right_need -= 1;
                }
            }
        }

        left_need + right_need
    }
}