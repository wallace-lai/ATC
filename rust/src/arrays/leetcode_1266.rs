struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(pts: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut curr = (pts[0][0], pts[0][1]);

        for i in 1..pts.len() {
            let next = (pts[i][0], pts[i][1]);
            let dx = (next.0 - curr.0).abs();
            let dy = (next.1 - curr.1).abs();
            let min = std::cmp::min(dx, dy);
            ans += min + if dx > min { dx - min } else { dy - min };
            curr = next;
        }

        ans
    }
}