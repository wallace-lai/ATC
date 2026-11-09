struct Solution;

impl Solution {
    // 法一：二维偏序最长链
    // 思路：消掉一维后做 LIS
    pub fn best_seq_at_index(height: Vec<i32>, weight: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 { return 0; }

        // 打包成 (身高，体重)
        let mut people: Vec<(i32, i32)> = height.into_iter()
            .zip(weight.into_iter())
            .collect();

        // 身高升序：身高相同，体重降序（关键）
        people.sort_by(|a, b| {
            a.0.cmp(&b.0).then(b.1.cmp(&a.1))
        });

        // 对体重要求严格 LIS
        let mut tails = Vec::with_capacity(n);
        for &(_, w) in people.iter() {
            let i = tails.partition_point(|&v| v < w);
            if i == tails.len() {
                tails.push(w);
            } else {
                tails[i] = w;
            }
        }

        tails.len() as i32
    }
}