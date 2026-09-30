struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        // 分类讨论
        // 1. 最终同一位置为奇数，则代价为处于偶数位的筹码个数
        // 证明：所有处于偶数位的筹码通过+1或者-1变换，全部变成
        // 奇数位，所有奇数位筹码通过+2或者-2变换变到同一位置
        // 
        // 2. 最终同一位置为偶数，则代价为处于奇数位的筹码个数
        // 证明：所有处于奇数位的筹码通过+1或者-1变换，全部变成
        // 偶数位，所有偶数位的筹码通过+2或者-2变换到同一位置
        // 
        // 所以处于偶数位和奇数位的筹码总数哪个更少，那个就是答案

        // 奇数位筹码总数
        let odd = position.iter().filter(|&p| *p & 1 == 1).count();
        let odd = odd as i32;
        std::cmp::min(odd, position.len() as i32 - odd)
    }
}