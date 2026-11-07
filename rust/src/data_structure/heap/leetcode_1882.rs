struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        // 初始化空闲服务器，保存 (服务器权重，服务器下标)
        let mut idle: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        for i in 0..servers.len() {
            idle.push(Reverse((servers[i], i)));
        }

        // 繁忙服务器堆，保存 (服务器完成任务的时刻，服务器下标)
        let mut busy: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();
        let mut now = 0u32;

        // 把已经完成任务的服务器从 busy 中移动到 idle 中
        fn release(
            idle: &mut BinaryHeap<Reverse<(i32, usize)>>,
            busy: &mut BinaryHeap<Reverse<(u32, usize)>>,
            now: u32,
            servers: &[i32]
        ) {
            while let Some(Reverse((t, i))) = busy.peek().copied() {
                if t > now { break; }
                busy.pop();
                idle.push(Reverse((servers[i], i)));
            }
        }

        // 依次处理每个任务
        let mut ans = vec![0; tasks.len()];
        for i in 0..tasks.len() {
            now = std::cmp::max(now, i as u32);
            release(&mut idle, &mut busy, now, &servers);
            // 如果此刻没有空闲服务器，则跳到 busy 的堆顶时刻
            if idle.is_empty() {
                if let Some(Reverse((t, i))) = busy.peek().copied() {
                    now = t;
                    release(&mut idle, &mut busy, now, &servers);
                }
            }
            if let Some(Reverse((_, k))) = idle.pop() {
                busy.push(Reverse((now + tasks[i] as u32, k)));
                ans[i] = k as i32;
            }
        }

        ans
    }
}