struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut v = vec![Vec::new(); 10];
        for num in nums {
            let max = {
                let mut ret = -1;
                let mut n = num;
                while n > 0 {
                    ret = ret.max(n % 10);
                    n /= 10;
                }
                ret
            };

            v[max as usize].push(num);
        }

        let mut ans = -1;
        for mut g in v {
            if g.len() < 2 { continue; }
            if g.len() > 2 {
                g.sort_unstable();
            }
            ans = ans.max(g[g.len() - 1] + g[g.len() - 2]);
        }

        ans
    }
}