struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：DFS
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        fn dfs(nums: &[i32], mut i: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                ans.push(t.clone());
                return;
            }

            // 把相同元素看成一组，比如 nums = [1, 1, 1, 2]，一个
            // 子集真正关心的是：选0个、1个、2个还是3个1，而不关心选
            // 的是第几个1

            // 当前的这个 x 已经选了，接下来考虑后面的元素。后面相同
            // 的 x 还可以继续选，用来表示选2个、3个相同的元素

            // 如果当前的这个 x 不选，那么后面那些相同的 x 也不能通过
            // 跳过当前，选后面的方式来选，否则会造成重复
    
            // 选
            let x = nums[i];
            t.push(x);
            dfs(nums, i + 1, t, ans);
            t.pop();

            // 不选
            i += 1;
            while i < nums.len() && nums[i] == x {
                i += 1;
            }
            dfs(nums, i, t, ans);
        }

        nums.sort_unstable();
        let mut t = vec![];
        let mut ans = vec![];
        dfs(&nums, 0, &mut t, &mut ans);
        ans
    }

    // 法二：通用解法，无论是否存在相同元素
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
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