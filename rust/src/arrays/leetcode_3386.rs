struct Solution;

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut index = events[0][0];
        let mut dtime = events[0][1];
        for i in 1..events.len() {
            let d = events[i][1] - events[i - 1][1];
            if d > dtime {
                dtime = d;
                index = events[i][0];
            } else if d == dtime && events[i][0] < index {
                index = events[i][0];
            }
        }
        index
    }
}