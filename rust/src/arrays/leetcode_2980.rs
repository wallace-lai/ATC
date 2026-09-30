struct Solution;

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        // 至少存在一个尾随零 ==> 按位或结果为偶数
        // 按位或结果为偶数 ==> 两个数均为偶数
        let mut even = 0;
        for num in nums {
            if num & 1 == 0 {
                even += 1;
                if even >= 2 {
                    return true;
                }
            }
        }

        false
    }
}