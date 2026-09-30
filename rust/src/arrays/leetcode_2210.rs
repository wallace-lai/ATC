struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut v = Vec::with_capacity(nums.len());
        for num in nums {
            if v.len() == 0 || v.last().unwrap() != &num {
                v.push(num);
            }
        }

        let mut ans = 0;
        for i in 1..(v.len() - 1) {
            if (v[i - 1] < v[i] && v[i] > v[i + 1]) ||
                (v[i - 1] > v[i] && v[i] < v[i + 1]) {
                ans += 1;
            }
        }
        ans
    }
}