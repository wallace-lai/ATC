struct Solution;

impl Solution {
    // pub fn minimum_pushes(word: String) -> i32 {
    //     let mut ans = 0;

    //     // 频次统计
    //     let mut count = [0; 26];
    //     for &c in word.as_bytes() {
    //         count[(c - b'a') as usize] += 1;
    //     }

    //     // 收集字符及其出现次数
    //     let mut chars_with_count: Vec<(u8, i32)> = (0..26)
    //         .filter(|&i| count[i] > 0)
    //         .map(|i|((b'a' + i as u8), count[i]))
    //         .collect();

    //     // 排序
    //     chars_with_count.sort_unstable_by(|a, b| {
    //         b.1.cmp(&a.1)
    //         // .then_with(|| a.0.cmp(&b.0))
    //     });

    //     println!("{:?}", chars_with_count);

    //     ans
    // }

    // 0ms - 击败100%
    pub fn minimum_pushes(word: String) -> i32 {
        let mut len = word.len() as i32;
        let mut ans = 0;
        let mut num = 1;

        while len >= 8 {
            ans += num * 8;
            num += 1;

            len -= 8;
        }

        ans += num * len;
        ans
    }
}