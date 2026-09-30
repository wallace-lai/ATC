struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|v| v[0]);

        let mut ans = 0;
        for i in 1..points.len() {
            if points[i][0] - points[i - 1][0] > ans {
                ans = points[i][0] - points[i - 1][0]
            }
        }

        ans
    }
}