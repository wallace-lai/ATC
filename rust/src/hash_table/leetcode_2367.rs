struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let len = nums.len();
        let mut ans = 0;

        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] + diff != nums[j] {
                    continue;
                }
                for k in (j + 1)..len {
                    if nums[j] + diff != nums[k] {
                        continue;
                    }

                    ans += 1;
                }
            }
        }

        ans
    }
}