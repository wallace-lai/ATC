struct Solution;

impl Solution {
    // 法一：O(n)，0ms，击败100%
    // pub fn is_monotonic(nums: Vec<i32>) -> bool {
    //     if nums.len() < 2 {
    //         return true;
    //     }

    //     // 跳过前面相等的部分
    //     let mut idx = 1;
    //     while idx < nums.len() {
    //         if nums[idx - 1] != nums[idx] {
    //             break;
    //         }
    //         idx += 1;
    //     }

    //     // 若数组元素全部相等
    //     if idx >= nums.len() {
    //         return true;
    //     }

    //     // 判断后面不相等的部分
    //     if nums[idx - 1] < nums[idx] {
    //         // 单调递增序列
    //         for i in idx..nums.len() {
    //             if nums[i - 1] > nums[i] {
    //                 return false;
    //             }
    //         }
    //     } else {
    //         // 单调递减序列
    //         for i in idx..nums.len() {
    //             if nums[i - 1] < nums[i] {
    //                 return false;
    //             }
    //         }
    //     }

    //     true
    // }

    // 法二：转换思路，代码简洁明了
    // O(n)，0ms，击败100%
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        // 假定数组既是单调递增的，也是单调递减的
        let mut inc = true;
        let mut dec = true;

        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                // 此时，数组不可能是单调递减的
                dec = false;
            } else if nums[i - 1] > nums[i] {
                // 此时，数组不可能是单调递增的
                inc = false;
            }
        }

        inc || dec
    }
}