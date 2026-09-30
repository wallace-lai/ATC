struct Solution;

impl Solution {
    // 法一：3ms 滑动窗口
    // pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    //     let n = customers.len();
    //     let satisfied: Vec<i32> = customers.iter().zip(grumpy.iter())
    //         .map(|(&c, &g)| if g == 1 { 0 } else { c })
    //         .collect();
        
    //     // pre[i]表示 sum(statisfied[0..i))
    //     let mut pre = vec![0; n + 1];
    //     for i in 1..pre.len() {
    //         pre[i] = satisfied[i - 1] + pre[i - 1];
    //     }

    //     let k = minutes as usize;
    //     let mut win = 0;
    //     let mut ans = 0;
    //     let mut left = 0;
    //     for right in 0..n {
    //         win += customers[right];
    //         if right + 1 < k { continue; }

    //         // 此时，窗口大小为k
    //         let sum = pre[left] + win + (pre[n] - pre[right + 1]);
    //         ans = ans.max(sum);

    //         win -= customers[left];
    //         left += 1;
    //     }

    //     ans
    // }

    // 法二：0ms，击败100%，滑动窗口
    // 转换思路，先计算不使用秘密技巧时能够感到满意的客户数，计算
    // 因为窗口的存在而增加的客户数，使得这个增加的客户数最大即可
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let n = customers.len();
        let total: i32 = customers.iter().zip(grumpy.iter())
            .map(|(&c, &g)| if g == 1 { 0 } else { c })
            .sum();

        let k = minutes as usize;
        let mut inc = 0;    // 因窗口的存在而增加的 最大 客户数
        let mut win = 0;    // 因窗口的存在而增加的客户数
        let mut left = 0;

        for right in 0..n {
            if grumpy[right] == 1 {
                win += customers[right];
            }
            if right + 1 < k { continue; }

            inc = inc.max(win);

            if grumpy[left] == 1 {
                win -= customers[left];
            }
            left += 1;
        }

        total + inc
    }
}
