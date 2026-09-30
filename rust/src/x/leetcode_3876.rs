struct Solution;

impl Solution {
    // 5ms
    // pub fn uniform_array(mut nums: Vec<i32>) -> bool {
    //     let n = nums.len();
    //     nums.sort_unstable();
    //     let (even, odd): (Vec<i32>, Vec<i32>) = nums
    //         .into_iter()
    //         .partition(|x| *x & 1 == 0);

    //     // println!("even is {:?}", even);
    //     // println!("odd is {:?}", odd);

    //     if even.len() == n || odd.len() == n { return true; }

    //     // 构造全奇
    //     if even[0] >= odd[0] + 1 { return true; }

    //     false
    // }

    pub fn uniform_array(nums: Vec<i32>) -> bool {
        // 搜索数组中的：偶数总数、奇数总数、最小偶数、最小奇数
        let mut even_count = 0;
        let mut odd_count = 0;
        let mut min_even = i32::MAX;
        let mut min_odd = i32::MAX;

        for &num in nums.iter() {
            if num & 1 == 1 {
                odd_count += 1;
                if num < min_odd { min_odd = num; }
            } else {
                even_count += 1;
                if num < min_even { min_even = num; }
            }
        }

        // 数组全奇或全偶，返true
        if even_count == nums.len() as i32 ||
            odd_count == nums.len() as i32 {
            return true;
        }

        // 数组同时存在奇数偶数时，无法构造全偶数组，此时
        // 能够成功构造全奇数组的条件是：对于最小的偶数min_even
        // 也存在一个奇数min_odd满足min_even >= min_odd + 1
        if min_even >= min_odd + 1 { return true; }

        false
    }
}