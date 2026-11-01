struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：深度优先搜索
    // 时间复杂度：O(n * 2^n)
    // 空间复杂度：O(n)
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut t = Vec::with_capacity(n);
        let mut ans = Vec::with_capacity(2usize.pow(n as u32));

        fn dfs(nums: &Vec<i32>, curr: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if curr == nums.len() {
                ans.push(t.clone());
                return;
            }

            // 选
            t.push(nums[curr]);
            dfs(nums, curr + 1, t, ans);
            t.pop();

            // 不选
            dfs(nums, curr + 1, t, ans);
        }

        dfs(&nums, 0, &mut t, &mut ans);
        ans
    }

    #[cfg(any())]
    // 法二：二进制位迭代，nums.len() < 32
    // 时间复杂度：O(n * 2^n)
    // 空间复杂度：O(n)
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut t = Vec::with_capacity(n);
        let mut ans = Vec::with_capacity(2usize.pow(n as u32));

        for mask in 0..(1usize << n) {
            t.clear();
            for i in 0..n {
                if (mask & (1usize << i)) != 0 {
                    t.push(nums[i]);
                }
            }
            ans.push(t.clone());
        }

        ans
    }

    // 法三：通用解法，无论是否存在相同元素
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        fn dfs(g: &[(i32, usize)], i: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if i == g.len() {
                ans.push(t.clone());
                return;
            }

            let (v, c) = g[i];
            // 对于这一组，选择 k 个，k 决定了从中选择多少个 v
            for k in 0..=c {
                let base = t.len();
                t.extend(std::iter::repeat(v).take(k));
                dfs(g, i + 1, t, ans);
                t.truncate(base);
            }
        }
    
        nums.sort_unstable();

        // 分组：(值，出现次数)
        let mut group: Vec<(i32, usize)> = Vec::new();
        for &x in nums.iter() {
            match group.last_mut() {
                Some((v, c)) if *v == x => { *c += 1 },
                _ => { group.push((x, 1)) },
            }
        }

        let mut t = vec![];
        let mut ans = vec![];
        dfs(&group, 0, &mut t, &mut ans);
        ans
    }
}