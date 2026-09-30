struct Solution;

impl Solution {
    // 超时
    // pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    //     let mut ans = -1;

    //     fn DFS(nums: &Vec<i32>, left: i32, right: i32, x: i32, count: i32, ans: &mut i32) {
    //         if x == 0 && (*ans == -1 || count < *ans) {
    //             *ans = count;
    //             return;
    //         }
    //         if left > right {
    //             return;
    //         }

    //         // 移除最左边元素
    //         if left < nums.len() as i32 &&
    //             nums[left as usize] <= x {
    //             DFS(nums, left + 1, right, x - nums[left as usize], count + 1, ans);
    //         }

    //         // 移除最右边元素
    //         if right >= 0 &&
    //             nums[right as usize] <= x {
    //             DFS(nums, left, right - 1, x - nums[right as usize], count + 1, ans);
    //         }
    //     }

    //     DFS(&nums, 0, nums.len() as i32 - 1, x, 0, &mut ans);

    //     ans
    // }

    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        if sum < x { return -1; }

        let mut right = 0;
        let mut lsum = 0;
        let mut rsum = sum;
        let mut ans = n  + 1;

        for left in -1..n {
            lsum += if left == -1 { 0 } else { nums[left as usize] };
            while right < n && lsum + rsum > x {
                rsum -= nums[right as usize];
                right += 1;
            }
            if lsum + rsum == x {
                ans = ans.min(left + 1 + n - right);
            }
        }

        if ans > n { -1 } else { ans }
    }
}
