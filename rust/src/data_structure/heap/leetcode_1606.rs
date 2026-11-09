struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let n = arrival.len();
        let k = k as usize;

        // 小优化：如果 k >= n，每台服务器恰好使用一次，前 n 台都是最繁忙的
        if k >= n {
            return (0..n).map(|x| x as i32).collect();
        }

        // 空闲服务器堆，保存空闲服务器的虚拟编号（最小堆）
        let mut idle: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for i in 0..k {
            idle.push(Reverse(i));
        }

        // 繁忙服务器堆，保存 (服务器完成请求的时刻, 服务器编号)（最小堆）
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut cnt = vec![0usize; k]; // cnt[i] 统计服务器 i 处理的请求数
        let mut max_cnt = 0usize;
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..n {
            let t = arrival[i] as i64;
            let i_mod_k = i % k;

            // 把已经完成请求的服务器从 busy 移至 idle
            while let Some(&Reverse((finish, j))) = busy.peek() {
                if finish > t {
                    break;
                }
                busy.pop();
                // 把服务器 j 在 [i, i+k) 中的同余值作为该服务器的虚拟编号
                let virtual_id = i + (j + k - i_mod_k) % k;
                idle.push(Reverse(virtual_id));
            }

            // 没有空闲服务器，舍弃该请求
            if idle.is_empty() {
                continue;
            }

            let Reverse(virtual_id) = idle.pop().unwrap();
            let j = virtual_id % k;
            let finish = t + load[i] as i64;
            busy.push(Reverse((finish, j)));

            cnt[j] += 1;
            if cnt[j] > max_cnt {
                max_cnt = cnt[j];
                ans = vec![j as i32];
            } else if cnt[j] == max_cnt {
                ans.push(j as i32);
            }
        }

        ans
    }
}