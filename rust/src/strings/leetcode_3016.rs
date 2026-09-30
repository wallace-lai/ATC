struct Solution;

impl Solution {
    // 3ms - 击败100%
    // pub fn minimum_pushes(word: String) -> i32 {
    //     // 频次统计
    //     let mut count = [0; 26];
    //     for &c in word.as_bytes() {
    //         count[(c - b'a') as usize] += 1;
    //     }

    //     // 收集字符及其出现次数
    //     let mut map: Vec<(u8, i32)> = (0..26)
    //         .filter(|&i| count[i] > 0)
    //         .map(|i| ((b'a' + i as u8), count[i]))
    //         .collect();
        
    //     // 排序
    //     map.sort_unstable_by(|a, b| {
    //         b.1.cmp(&a.1)
    //     });

    //     // println!("map is {:?}", map);

    //     let ans = map.chunks(8)
    //         .enumerate()
    //         .map(|(idx, chunk)| {
    //             let group_num = (idx + 1) as i32;
    //             chunk.iter().map(|(_, num)| num * group_num).sum::<i32>()
    //         })
    //         .sum();

    //     ans
    // }

    // 0ms - 更简洁的版本
    pub fn minimum_pushes(word: String) -> i32 {
        let mut count = [0; 26];
        for &c in word.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }
        count.sort_unstable_by(|a, b| { b.cmp(&a) });

        let mut ans: i32 = 0;
        for i in 0..count.len() {
            if count[i] > 0 {
                ans += (i as i32 / 8 + 1) * count[i];
            }
        }

        ans
    }
}