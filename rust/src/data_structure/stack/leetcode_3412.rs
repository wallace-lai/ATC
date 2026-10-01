struct Solution;

impl Solution {
    // 7ms
    // 使用26个栈，每个字母一个
    // pub fn calculate_score(s: String) -> i64 {
    //     let b = s.as_bytes();
    //     let n = b.len();
    //     let mut stks: Vec<Vec<usize>>= vec![vec![]; 26];

    //     let mut ans = 0i64;
    //     for i in 0..n {
    //         let posi = (b[i] - b'a') as usize;
    //         let posj = 25 - posi;
    //         if stks[posj].len() > 0 {
    //             let &j = stks[posj].last().unwrap();
    //             ans += (i - j) as i64;
    //             stks[posj].pop();
    //         } else {
    //             stks[posi].push(i);
    //         }
    //     }

    //     ans
    // }

    // 7ms
    pub fn calculate_score(s: String) -> i64 {
        let mut stks: Vec<Vec<usize>> = vec![vec![]; 26];

        s.as_bytes()
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, &c)| {
                let posi = (c - b'a') as usize;
                let posj = 25 - posi;
                if let Some(j) = stks[posj].pop() {
                    acc + (i - j) as i64
                } else {
                    stks[posi].push(i);
                    acc
                }
            })
    }
}
