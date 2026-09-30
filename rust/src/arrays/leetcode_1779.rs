struct Solution;

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut ans = -1;
        let mut dis = i32::MAX;

        for (i, p) in points.iter().enumerate() {
            if p[0] == x || p[1] == y {
                let tmp = (x - p[0]).abs() + (y - p[1]).abs();
                if tmp < dis {
                    dis = tmp;
                    ans = i as i32;
                }
            }
        }

        ans
    }
}