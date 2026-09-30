struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last = 0;   // last保存窗口当中第一个0所在的位置 + 1
        let mut left = 0;

        // right自增扩展窗口
        for right in 0..nums.len() {
            if nums[right] == 0 {
                if last == 0 {          // 首次遇到0
                    last = right + 1;
                } else if last > left { // 第二次遇到0，此时需要移动left到第一个0的后面那个位置
                    // 缩减窗口一步到位
                    left = last;
                    last = right + 1;
                }
            }
            ans = ans.max(right - left);    // 因为必须删掉一个元素，所以不能是right - left + 1
        }

        ans as i32
    }
}
