struct Solution;

impl Solution {
    pub fn cut_squares(square1: Vec<i32>, square2: Vec<i32>) -> Vec<f64> {
        // 求中心坐标
        let (c1x, c1y) = (
            square1[0] as f64 + square1[2] as f64 / 2.0,
            square1[1] as f64 + square1[2] as f64 / 2.0,
        );
        let (c2x, c2y) = (
            square2[0] as f64 + square2[2] as f64 / 2.0,
            square2[1] as f64 + square2[2] as f64 / 2.0,
        );

        // 情况1：中心x坐标相同，竖直线
        if (c1x - c2x).abs() < 1e-9 {
            let y1 = square1[1].min(square2[1]) as f64;
            let y2 = (square1[1] + square1[2]).max(square2[1] + square2[2]) as f64;
            return vec![c1x, y1, c2x, y2];
        }

        // 计算斜率和截距
        let k = (c1y - c2y) / (c1x - c2x);
        let b = c1y - k * c1x;

        let mut p1;
        let mut p2;

        // 情况2：根据斜率绝对值分类
        if k.abs() <= 1.0 {
            // 与左右边相交
            let left_x = square1[0].min(square2[0]) as f64;
            let right_x = (square1[0] + square1[2]).max(square2[0] + square2[2]) as f64;
            p1 = (left_x, k * left_x + b);
            p2 = (right_x, k * right_x + b);
        } else {
            // 与上下边相交
            let bottom_y = square1[1].min(square2[1]) as f64;
            let top_y = (square1[1] + square1[2]).max(square2[1] + square2[2]) as f64;
            p1 = ((bottom_y - b) / k, bottom_y);
            p2 = ((top_y - b) / k, top_y);
        }

        // 排序：保证 X1 < X2，或 X1 == X2 时 Y1 <= Y2
        if p1.0 > p2.0 || (p1.0 == p2.0 && p1.1 > p2.1) {
            std::mem::swap(&mut p1, &mut p2);
        }

        vec![p1.0, p1.1, p2.0, p2.1]
    }
}