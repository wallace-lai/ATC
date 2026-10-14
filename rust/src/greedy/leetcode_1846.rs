struct Solution;

impl Solution {
    // 时间复杂度：O(nlogn + n)
    // 空间复杂度：O(logn)
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();

        let mut x = 0;
        for i in 0..arr.len() {
            if arr[i] - x > 1 {
                arr[i] = x + 1;
            }
            x = arr[i];
        }

        x
    }
}
