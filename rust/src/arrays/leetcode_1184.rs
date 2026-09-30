struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;

        // 顺时针
        let mut d1 = 0;
        let mut curr = start;
        while curr != destination {
            d1 += distance[curr as usize];
            curr = (curr + 1) % n;
        }

        // 逆时针
        let mut d2 = 0;
        curr = start;
        while curr != destination {
            let prev = (curr - 1 + n) % n;
            d2 += distance[prev as usize];
            curr = prev;
        }

        std::cmp::min(d1, d2)
    }
}