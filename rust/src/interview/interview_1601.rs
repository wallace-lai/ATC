struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：加法
    pub fn swap_numbers(mut num: Vec<i32>) -> Vec<i32> {
        num[0] = num[0] + num[1];
        num[1] = num[0] - num[1];
        num[0] = num[0] - num[1];
        num
    }

    // 法二：异或
    pub fn swap_numbers(mut num: Vec<i32>) -> Vec<i32> {
        num[0] = num[0] ^ num[1];
        num[1] = num[0] ^ num[1];
        num[0] = num[0] ^ num[1];
        num
    }
}