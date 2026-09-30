struct Solution;

impl Solution {
    pub fn check_overlap(r: i32, xp: i32, yp: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let mut d: u64 = 0;
        if xp < x1 || xp > x2 {
            d += std::cmp::min(
                (x1 - xp) as u64 * (x1 - xp) as u64,
                (x2 - xp) as u64 * (x2 - xp) as u64
            );
        }
        if yp < y1 || yp > y2 {
            d += std::cmp::min(
                (y1 - yp) as u64 * (y1 - yp) as u64,
                (y2 - yp)as u64 * (y2 - yp) as u64
            );
        }

        d <= r as u64 * r as u64
    }
}