struct Solution;

impl Solution {
    // 超时
    // pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    //     let mut ans = vec![0; queries.len()];

    //     fn dfs(nums: &Vec<i32>, queries: &Vec<i32>, ans: &mut Vec<i32>, idx: usize, sum: i32, len: i32) {
    //         if idx == nums.len() {
    //             for i in 0..queries.len() {
    //                 if sum <= queries[i] && len > ans[i] {
    //                     ans[i] = len;
    //                 }
    //             }
    //             return;
    //         }

    //         dfs(nums, queries, ans,  idx + 1, sum, len);
    //         dfs(nums, queries, ans, idx + 1, sum + nums[idx], len + 1);
    //     }

    //     dfs(&nums, &queries, &mut ans, 0, 0, 0);
    //     ans
    // }

    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut pre = vec![0; nums.len()];
        pre[0] = nums[0];
        for i in 1..pre.len() {
            pre[i] = nums[i] + pre[i - 1];
        }

        let mut ans = vec![0; queries.len()];
        for i in 0..queries.len() {
            if queries[i] >= pre[pre.len() - 1] {
                ans[i] = pre.len() as i32;
                continue;
            }

            for k in 0..pre.len() {
                if pre[k] > queries[i] {
                    ans[i] = k as i32;
                    break;
                }
            }
        }

        ans
    }
}