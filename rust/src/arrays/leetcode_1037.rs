struct Solution;

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let a = (points[0][0], points[0][1]);
        let b = (points[1][0], points[1][1]);
        let c = (points[2][0], points[2][1]);
        if a == b || b == c || c == a { return false; }

        let vab = (b.0 - a.0, b.1 - a.1);
        let vac = (c.0 - a.0, c.1 - a.1);
        let d = vab.0 as f64 * vac.1 as f64 -
            vab.1 as f64 * vac.0 as f64;
        if d.abs() < 1e-6 { return false; }

        true
    }
}