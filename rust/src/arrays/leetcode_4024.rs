struct Solution;

impl Solution {
    pub fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
        let n = drones.len();
        let mut ans = -1;
        let mut mind = 0;
        for i in 0..n {
            let d = (drones[i][0] - target[0]).abs() +
                (drones[i][1] - target[1]).abs();
            if d > drones[i][2] { continue; }

            if ans == -1 || d < mind {
                ans = i as i32;
                mind = d;
            }
        }
        ans
    }
}