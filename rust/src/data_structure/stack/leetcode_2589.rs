struct Solution;

impl Solution {
    // 法一：贪心
    // time: O(nlogn + nU), U = max(end_i)
    // space: O(U)
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1]);
        let len = tasks[tasks.len() - 1][1] as usize + 1;
        let mut run = vec![false; len]; // 标记第i个时间段是否运行
        let mut sum = 0;    // 目前为止所有运行的时间段的总数

        for t in tasks.iter() {
            let start = t[0] as usize;
            let end = t[1] as usize;
            let mut duration = t[2];

            // 由于电脑能同时运行无数个任务，所以当前任务
            // 的duration需要去掉目前为止已运行的时间段
            let slice = &run[start..=end];
            let slice_sum: i32 = slice.iter()
                .map(|&r| if r == true { 1 } else { 0 })
                .sum();
            duration -= slice_sum;
            if duration <= 0 {
                continue;   // 当前任务已完成
            }

            // 当前任务尚未完成，从后往前找没有运行的时间点
            for i in (start..=end).rev() {
                if run[i] { continue; }
                run[i] = true;
                sum += 1;
                duration -= 1;
                if duration == 0 {
                    break;
                }
            }
        }

        sum
    }
}