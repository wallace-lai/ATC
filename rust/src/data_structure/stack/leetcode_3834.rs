struct Solution;

impl Solution {
    // 0ms
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut v = Vec::with_capacity(nums.len());

        for n in nums {
            let mut n = n as i64;
            while v.len() > 0 && v[v.len() - 1] == n {
                v.pop();
                n += n;
            }
            v.push(n);
        }

        v
    }
}