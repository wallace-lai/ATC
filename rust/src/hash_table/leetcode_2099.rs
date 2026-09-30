struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut v: Vec<(usize, i32)> = nums.iter()
            .enumerate()
            .map(|(i, num)| (i, *num))
            .collect();

        v.sort_unstable_by_key(|(_, num)| -*num);
        let slice = &mut v[0..k as usize];
        slice.sort_unstable_by_key(|(i, _)| *i);
        // println!("v is {:?}", v);

        slice.iter().map(|(_, num)| *num).collect()
    }
}