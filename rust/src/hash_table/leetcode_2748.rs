struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        const COPRIME: [&[u8]; 10] = [
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0], // 1
            &[0, 1, 0, 3, 0, 5, 0, 7, 0, 9, 0, 0, 0, 0, 0, 0], // 2
            &[0, 1, 2, 0, 4, 5, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0], // 3
            &[0, 1, 0, 3, 0, 5, 0, 7, 0, 9, 0, 0, 0, 0, 0, 0], // 4
            &[0, 1, 2, 3, 4, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0], // 5
            &[0, 1, 0, 0, 0, 5, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0], // 6
            &[0, 1, 2, 3, 4, 5, 6, 0, 8, 9, 0, 0, 0, 0, 0, 0], // 7
            &[0, 1, 0, 3, 0, 5, 0, 7, 0, 9, 0, 0, 0, 0, 0, 0], // 8
            &[0, 1, 2, 0, 4, 5, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0], // 9
        ];

        let mut first_and_last = vec![(0, 0); nums.len()];
        for (i, &num) in nums.iter().enumerate() {
            let first = {
                let mut n = num;
                while n >= 10 {
                    n /= 10;
                }
                n
            };
            let last = num % 10;
            first_and_last[i].0 = first;
            first_and_last[i].1 = last;
        }

        let mut ans = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let a = first_and_last[i].0;
                let b = first_and_last[j].1;
                // println!("a is {a}, b is {b}");

                let slice = COPRIME[a as usize];
                if slice[b as usize] > 0 { ans += 1; }
            }
        }

        ans
    }
}