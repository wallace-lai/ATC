struct Solution;

impl Solution {
    pub fn intersection(start1: Vec<i32>, end1: Vec<i32>, start2: Vec<i32>, end2: Vec<i32>) -> Vec<f64> {
        const EPS: f64 = 1e-8;

        let p1 = (start1[0] as f64, start1[1] as f64);
        let p2 = (end1[0] as f64, end1[1] as f64);
        let p3 = (start2[0] as f64, start2[1] as f64);
        let p4 = (end2[0] as f64, end2[1] as f64);

        // 向量辅助运算
        let sub = |a: (f64, f64), b: (f64, f64)| (a.0 - b.0, a.1 - b.1);
        let add = |a: (f64, f64), b: (f64, f64)| (a.0 + b.0, a.1 + b.1);
        let mul = |a: (f64, f64), k: f64| (a.0 * k, a.1 * k);
        let cross = |a: (f64, f64), b: (f64, f64)| a.0 * b.1 - a.1 * b.0;

        // 点 p 是否在线段 ab 上（含端点，含退化点）
        let on_segment = |p: (f64, f64), a: (f64, f64), b: (f64, f64)| -> bool {
            let ab = sub(b, a);
            let ap = sub(p, a);
            if cross(ab, ap).abs() > EPS {
                return false;   // 不共线
            }
            // 包围盒检查
            p.0 >= a.0.min(b.0) - EPS && p.0 <= a.0.max(b.0) + EPS
                && p.1 >= a.1.min(b.1) - EPS && p.1 <= a.1.max(b.1) + EPS
        };

        let d1 = sub(p2, p1);
        let d2 = sub(p4, p3);
        let d3 = sub(p3, p1);

        let denom = cross(d1, d2);

        if denom.abs() < EPS {
            // —— 情况 B：平行但不共线 ——
            if cross(d3, d1).abs() > EPS {
                return vec![];
            }
            // —— 情况 C：共线，检查四端点 ——
            for p in [p1, p2, p3, p4] {
                if on_segment(p, p1, p2) && on_segment(p, p3, p4) {
                    return vec![p.0, p.1];
                }
            }
            return vec![]; // 共线但无重叠
        }

        // —— 情况 A：不平行，解参数 ——
        let t = cross(d3, d2) / denom;
        let s = cross(d3, d1) / denom;

        if t >= -EPS && t <= 1.0 + EPS && s >= -EPS && s <= 1.0 + EPS {
            let res = add(p1, mul(d1, t));
            return vec![res.0, res.1];
        }

        vec![]
    }
}