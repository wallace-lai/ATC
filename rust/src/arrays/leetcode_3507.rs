struct Solution;

impl Solution {
    // 判断数组v是否为非递减
    pub fn is_asc(v: &[i32]) -> bool {
        for i in 1..v.len() {
            if v[i - 1] > v[i] {
                return false;
            }
        }

        true
    }

    // 0ms，击败100%
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return 0; }

        let mut ans = 0;
        while !Self::is_asc(&nums) {
            ans += 1;

            let mut pair_idx = 0;
            let mut pair_sum = nums[0] + nums[1];
            for i in 0..(nums.len() - 1) {
                let sum = nums[i] + nums[i + 1];
                if sum < pair_sum {
                    pair_idx = i;
                    pair_sum = sum;
                }
            }

            nums[pair_idx] = pair_sum;
            nums.remove(pair_idx + 1);
        }

        ans
    }
}