struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        // 保存当前排队人原先的下标
        let mut queue: VecDeque<usize> = (0..tickets.len()).collect();

        let mut ans = 0;
        while tickets[k] > 0 {
            let &front = queue.front().unwrap();
            tickets[front] -= 1;
            queue.pop_front();
            if tickets[front] > 0 {
                queue.push_back(front);
            }
            ans += 1;

            // for &i in queue.iter() {
            //     print!("{} ", tickets[i]);
            // }
            // println!();
        }

        ans
    }
}