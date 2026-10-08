struct Solution;

impl Solution {
    // 法一：使用栈
    // 464ms，击败0%
    // pub fn remove_duplicates(s: String, k: i32) -> String {
    //     let k = k as usize;
    //     let mut v = Vec::with_capacity(s.len());

    //     for &c in s.as_bytes().iter() {
    //         v.push(c);
    //         if v.len() >= k {
    //             let slice = &v[v.len() - k..];
    //             if slice.windows(2).all(|w| w[0] == w[1]) {
    //                 v.truncate(v.len() - k);
    //             }
    //         }
    //     }

    //     unsafe { String::from_utf8_unchecked(v) }
    // }

    // 法二：栈中保存字符和出现次数的键值对
    // 0ms，击败100%
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut v = Vec::with_capacity(s.len());

        for c in s.chars() {
            match v.last_mut() {
                Some((top, cnt)) if *top == c => {
                    if *cnt == k - 1 {
                        v.pop();
                    } else {
                        *cnt += 1;
                    }
                },
                _ => {
                    v.push((c, 1));
                }
            }
        }

        let mut ans = String::with_capacity(s.len());
        for (c, n) in v {
            ans.extend(std::iter::repeat(c).take(n as usize))
        }

        ans
    }
}