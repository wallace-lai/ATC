struct Solution;

impl Solution {
    #[cfg(any())]
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let checker = |beg: usize, end: usize| -> bool {
            let mut inc = true;
            let mut dec = true;

            for i in beg..end {
                if nums[i] >= nums[i + 1] {
                    inc = false;
                    break;
                }
            }
            for i in beg..end {
                if nums[i] <= nums[i + 1] {
                    dec = false;
                }
            }

            inc || dec
        };

        for d in (0..n).rev() {
            for i in 0..n {
                if i + d >= n { break; }
                if checker(i, i + d) {
                    return d as i32 + 1;
                }
            }
        }

        unreachable!()
    }

    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut i = 0;
        let mut ans = 1;
        while i < n - 1 {
            if nums[i + 1] == nums[i] {
                i += 1;
                continue;
            }

            let j = i;
            let inc = nums[i + 1] > nums[i];
            i += 2;
            while i < n && nums[i] != nums[i - 1] && ((nums[i] > nums[i - 1]) == inc) {
                i += 1;
            }

            ans = ans.max(i - j);
            i -= 1;
        }
    

        ans as i32
    }
}