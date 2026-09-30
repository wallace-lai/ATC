struct Solution;

impl Solution {
    // 11ms，性能差
    // pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    //     let v: HashSet<i32> = nums.into_iter().collect();
    //     let mut d = 0;

    //     loop {
    //         if v.contains(&d) { return d; }
    //         if v.contains(&(-d)) { return -d; }
    //         d += 1;
    //     }
    // }

    // 0ms
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        for num in nums {
            if num.abs() < ans.abs() ||
                (num.abs() == ans.abs() && num > ans) {
                ans = num;
            }
        }
        ans
    }
}