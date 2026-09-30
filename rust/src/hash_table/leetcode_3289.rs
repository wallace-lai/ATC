struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut count = [0; 128];
        for num in nums {
            count[num as usize] += 1;
        }

        let mut ans = Vec::with_capacity(2);
        for (i, num) in count.into_iter().enumerate() {
            if num == 2 { ans.push(i as i32); }
        }

        ans
    }
}