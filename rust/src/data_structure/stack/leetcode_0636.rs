struct Solution;

impl Solution {
    // WA
    // pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    //     let n = n as usize;
    //     let mut ans = vec![0; n];

    //     struct NormalizedLog {
    //         id: u32,
    //         timestamp: u32,
    //         start: bool
    //     }

    //     let mut normalized_logs = Vec::with_capacity(logs.len());
    //     for log in logs.iter() {
    //         let l: Vec<&str> = log.split(':').collect();
    //         assert_eq!(l.len(), 3);
    //         if let Ok(id) = l[0].parse::<u32>() &&
    //             let Ok(timestamp) = l[2].parse::<u32>() {
    //             normalized_logs.push(NormalizedLog {
    //                 id: id,
    //                 timestamp: timestamp,
    //                 start: if l[1] == "start" { true } else { false }
    //             });
    //         }
    //     }

    //     for i in 1..normalized_logs.len() {
    //         let prev = &normalized_logs[i - 1];
    //         let curr = &normalized_logs[i];
    //         match (prev.start, curr.start) {
    //             (true, true) => {
    //                 let time = (curr.timestamp - prev.timestamp) as i32;
    //                 ans[prev.id as usize] += time;
    //             },
    //             (true, false) => {
    //                 let time = (curr.timestamp - prev.timestamp + 1) as i32;
    //                 ans[prev.id as usize] += time;
    //             },
    //             (false, false) => {
    //                 let time = (curr.timestamp - prev.timestamp) as i32;
    //                 ans[curr.id as usize] += time;
    //             },
    //             _ => {}
    //         }
    //     }

    //     ans
    // }

    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n];
        let mut stk: Vec<(u32, u32)> = Vec::new();

        for log in logs.iter() {
            let l: Vec<&str> = log.split(':').collect();
            assert_eq!(l.len(), 3);
            if let Ok(id) = l[0].parse::<u32>() &&
                let Ok(timestamp) = l[2].parse::<u32>() {
                if l[1] == "start" {
                    if stk.len() > 0 {
                        let top = stk.last_mut().unwrap();
                        ans[top.0 as usize] += (timestamp - top.1) as i32;
                        top.1 = timestamp;
                    }
                    stk.push((id, timestamp));
                } else {
                    let &top = stk.last().unwrap();
                    stk.pop();
                    ans[top.0 as usize] += (timestamp - top.1 + 1) as i32;
                    if stk.len() > 0 {
                        let new_top = stk.last_mut().unwrap();
                        new_top.1 = timestamp + 1;
                    }
                }
            }
        }

        ans
    }
}