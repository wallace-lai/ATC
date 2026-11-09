struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {

        fn dfs(nums: &[i32], curr: usize, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
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

        let n = nums.len();
        let mut t = Vec::with_capacity(n);
        let mut ans = Vec::with_capacity(2usize.pow(n as u32));

        dfs(&nums, 0, &mut t, &mut ans);
        ans
    }
}