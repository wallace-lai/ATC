struct Solution;

impl Solution {
    // 法一：斜率哈希法
    // 思路：
    // 1. 固定锚点：遍历每一个点，将其作为锚点
    // 2. 计算斜率：对于锚点之后的每一个点，计算它们与锚点构成的直线斜率
    // 3. 哈希表统计：使用哈希表记录相同斜率出现的次数，次数表示斜率为 k 的除锚点以外的点数
    // 4. 更新答案：对于每个锚点，找到哈希表中计数最大的斜率，该直线上的点数为计数值 + 1
    pub fn best_line(points: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let n = points.len();
        let mut best = vec![0, 1];
        let mut max_points = 2;

        fn gcd(mut a: i32, mut b: i32) -> i32 {
            if a == 0 && b == 0 { return 1; }
            while b != 0 {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        for i in 0..n {
            // 存储共线点的下标列表
            let mut map: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

            for j in (i + 1)..n {
                let mut dx = points[j][0] - points[i][0];
                let mut dy = points[j][1] - points[i][1];

                // 重合点跳过
                if dx == 0 && dy == 0 { continue; }
                
                // 约分化到最简
                let g = gcd(dx.abs(), dy.abs());
                dx /= g;
                dy /= g;

                // 规范化方向，确保 (dx, dy) 和 (-dx, -dy) 被视为同一斜率
                if dx < 0 || (dx == 0 && dy < 0) {
                    dx = -dx;
                    dy = -dy;
                }

                map.entry((dx, dy)).or_default().push(j);
            }

            // 在当前锚点 i 的所有斜率中，找到点数最多的
            for (_, mut idx) in map {
                let total = idx.len() + 1;  // 加上锚点 i 本身
                idx.push(i);
                idx.sort_unstable();

                if total > max_points {
                    max_points = total;
                    best = vec![idx[0] as i32, idx[1] as i32];
                } else if total == max_points {
                    // 并列时，比较字典序
                    let cand = vec![idx[0] as i32, idx[1] as i32];
                    if cand[0] < best[0] || (cand[0] == best[0] && cand[1] < best[1]) {
                        best = cand;
                    }
                }
            }
        }

        best
    }
}