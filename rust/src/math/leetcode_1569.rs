struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        // 0 和 n+1 作为哨兵结点
        let n = nums.len();
        let mut prev = vec![0; n + 2];  // 前驱
        let mut next = vec![0; n + 2];  // 后继
        let mut size = vec![0; n + 2];  // 子树大小
        for i in 0..n + 2 {
            prev[i] = i - 1;
            next[i] = i + 1;
        }

        let mut prod = 1;   // 分母

        // 自底向上计算每棵子树的大小
        for x in nums.into_iter().rev() {
            let x = x as usize;
            size[x] = 1;

            // x 左右两侧最近的未被删除节点，即为 x 的左右儿子
            let left = prev[x];
            if size[left] > 0 {
                // 加上子树的大小
                size[x] += size[left];
                // 删除 left：在 left 的前一个节点与x之间建立双向链接
                let pl = prev[left];
                prev[x] = pl;
                next[pl] = x;
            }

            let right = next[x];
            if size[right] > 0 {
                // 加上右子树的大小
                size[x] += size[right];
                // 删除 right：在 right 的下一个节点与 x 之间建立双向链接
                let nr = next[right];
                prev[nr] = x;
                next[x] = nr;
            }

            prod = prod * size[x] as i64 % MOD;
        }

        // 计算分子 n!
        let mut fac = 1;
        for i in 2..n + 1 {
            fac = fac * i as i64 % MOD;
        }

        ((fac * Self::qpow(prod, MOD - 2) - 1 + MOD) % MOD) as i32
    }

    fn qpow(mut x: i64, mut n: i64) -> i64 {
        let mut res = 1;
        while n > 0 {
            if n & 1 == 1 {
                res = res * x % MOD;
            }
            x = x * x % MOD;
            n /= 2;
        }
        res
    }
}