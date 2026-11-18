struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：排序后交换（O(nlogn)）
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort();

        // let mut i = 0;  // 峰 - 谷 - 峰 - 谷
        // let mut i = 1;  // 谷 - 峰 - 谷 - 峰

        let mut i = 0;
        while i + 1 < nums.len() {
            nums.swap(i, i + 1);
            i += 2;
        }
    }

    // 法二：贪心（O(n)）
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        for i in 0..n.saturating_sub(1) {
            let need_swap = if i % 2 == 0 {
                // 谷位：要求 nums[i] <= nums[i + 1]
                nums[i] > nums[i + 1]
            } else {
                // 峰位：要求 nums[i] >= nums[i + 1]
                nums[i] < nums[i + 1]
            };
            if need_swap {
                nums.swap(i, i + 1);
            }
        }
    }
}