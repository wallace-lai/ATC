struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        fn area(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> Option<f64> {
            let vab = (b.0 - a.0, b.1 - a.1);
            let vac = (c.0 - a.0, c.1 - a.1);
            let d = vab.0 as f64 * vac.1 as f64 -
                vab.1 as f64 * vac.0 as f64;
            if d.abs() < 1e-6 { return None; }

            Some(d.abs() / 2.0)
        }

        let mut ans = 0.0;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    let a = (points[i][0], points[i][1]);
                    let b = (points[j][0], points[j][1]);
                    let c = (points[k][0], points[k][1]);
                    if let Some(area) = area(a, b, c) {
                        if area > ans { ans = area; }
                    }
                }
            }
        }

        ans
    }
}