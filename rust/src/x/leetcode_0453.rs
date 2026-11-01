struct Solution;

impl Solution {
    // 法一：数学
    // 题目只要求所有的数相等，不一定非要等于某个具体的数。所以
    // 只需要考虑所有数之间的相对差值即可。让 n-1 个数加一和让
    // 一个数减一对相对差值的影响是一样的，所以二者等价。问题相
    // 当于每次选一个数减去一，使得所有数相等的最小操作数是多少
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let &min = nums.iter().min().unwrap();
        let mut ans = 0;
        for &num in nums.iter() {
            ans += num - min;
        }
        ans
    }
}