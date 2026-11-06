struct Solution;

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, mut queries: Vec<i32>) -> Vec<i32> {
        // 日志按照时间升序排列
        logs.sort_unstable_by_key(|log| log[1]);

        // 查询按照时间升序排列
        let mut idx: Vec<usize> = (0..queries.len()).collect();
        idx.sort_unstable_by_key(|&i| queries[i]);

        // 滑动窗口要维护的信息
        let mut no_recv = n;    // 没有收到请求的服务器数量
        let mut count = vec![0; n as usize + 1];    // 每个服务器收到的请求个数

        let mut left = 0;
        let mut right = 0;

        for id in idx {
            let q = queries[id];

            // 扩展窗口的条件：日志发生的时间小于等于当前查询的时间 q
            while right < logs.len() && logs[right][1] <= q {
                // 收到请求的服务器id
                let sid = logs[right][0] as usize;
                if count[sid] == 0 { no_recv -= 1; }
                count[sid] += 1;
                right += 1;
            }

            // 收缩窗口的条件：日志发生的时间小于 q - x
            while left < logs.len() && logs[left][1] < q - x {
                let sid = logs[left][0] as usize;
                count[sid] -= 1;
                if count[sid] == 0 { no_recv += 1; }
                left += 1;
            }

            // 当前的窗口就是时间段 [q - x, q] 的情况
            queries[id] = no_recv;
        }

        queries
    }
}