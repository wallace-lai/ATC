struct Solution;

impl Solution {
    pub fn generate_combinations(n: u32, c: u32) -> Vec<Vec<u32>> {
        // 从 start..n 中挑选 c 个物品，结果保存在comb中
        fn dfs(n: u32, c: u32, start: u32, path: &mut Vec<u32>, comb: &mut Vec<Vec<u32>>) {
            if path.len() == c as usize {
                comb.push(path.clone());
                return;
            }

            // 需要挑选c个物品，已经挑选了path.len()个，还需挑选的个数
            let need = c - path.len() as u32;

            // 选完i之后，还剩need - 1个物品要选，则索引i后面至少还要
            // 有need - 1个物品可供选择，即：n - (i + 1) >= need - 1
            // ==> n - 1 - i >= need - 1
            // ==> i <= n - need
            for i in start..=(n - need) {
                path.push(i);   // 选择第i个物品
                dfs(n, c, i + 1, path, comb);
                path.pop();     // 回溯
            }
        }

        let mut comb = vec![];
        let mut path = vec![];
        dfs(n, c, 0, &mut path, &mut comb);
        comb
    }

    pub fn dfs(v: &Vec<i32>, c: i32, t: i32, p: &Vec<i32>, start: i32, choose: &mut Vec<i32>, sum: i32, ans: &mut i32) {
        if choose.len() == c as usize {
            *ans = (*ans + 1) % (1000000007);
            return;
        }

        let n = v.len() as i32;
        let need = c - choose.len() as i32;

        // 剪枝：剩余元素不够
        if start + need > n { return; }

        // 剪枝：即使从start取最小的need个，总和也超限
        let sum_min = p[(start + need) as usize] - p[start as usize];
        if sum + sum_min > t { return; }

        // 遍历可选的下一个物品
        for i in start..=(n - need) {
            // 剪枝：价值升序排列，若当前物品加入超限，后续价值会更大更会超限
            let new_sum = sum + v[i as usize];
            if new_sum > t { break; }

            choose.push(i);
            Self::dfs(v, c, t, p, i + 1, choose, new_sum, ans);
            choose.pop();
        }
    }

    // 超时
    // pub fn purchase_plans(mut nums: Vec<i32>, target: i32) -> i32 {
    //     nums.sort_unstable();

    //     let mut prefix: Vec<i32> = vec![0; nums.len() + 1];
    //     for i in 0..nums.len() {
    //         prefix[i + 1] = prefix[i] + nums[i];
    //     }

    //     let mut ans = 0;
    //     let mut choose = vec![];
    //     Self::dfs(&nums, 2, target, &prefix, 0, &mut choose, 0, &mut ans);

    //     ans
    // }

    // 14ms，击败100%
    pub fn purchase_plans(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut ans = 0_u64;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] + nums[right] > target {
                right -= 1;
            } else {
                ans += (right - left) as u64;
                left += 1;
            }
        }

        (ans % 1000000007) as i32
    }
}