struct Solution;

impl Solution {
    pub fn hardest_worker(_: i32, logs: Vec<Vec<i32>>) -> i32 {
        let (mut id, mut time) = (logs[0][0], logs[0][1]);
        for i in 1..logs.len() {
            let dtime = logs[i][1] - logs[i - 1][1];
            if dtime > time {
                time = dtime;
                id = logs[i][0];
            } else if dtime == time && logs[i][0] < id {
                id = logs[i][0];
            }
        }

        id
    }
}