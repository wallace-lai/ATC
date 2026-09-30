struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(nums.len() * 3);
        let mut tmp = vec![0; 8];

        for num in nums {
            tmp.clear();

            let mut n = num;
            while n > 0 {
                tmp.push(n % 10);
                n /= 10;
            }

            v.extend(tmp.iter().rev());
        }

        v
    }
}