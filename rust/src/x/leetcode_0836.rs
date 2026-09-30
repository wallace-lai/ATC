struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        struct Point { x: i32, y: i32 }
        let a = Point { x: rec1[0], y: rec1[1] };
        let b = Point { x: rec1[2], y: rec1[3] };
        let c = Point { x: rec2[0], y: rec2[1] };
        let d = Point { x: rec2[2], y: rec2[3] };

        if c.x >= b.x || c.y >= b.y ||
            a.x >= d.x || a.y >= d.y {
            return false;
        }

        true
    }
}