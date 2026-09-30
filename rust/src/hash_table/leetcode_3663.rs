struct Solution;

impl Solution {
    pub fn get_least_frequent_digit(mut n: i32) -> i32 {
        let mut count = [0; 10];
        while n > 0 {
            let idx = (n % 10) as usize;
            count[idx] += 1;
            n /= 10;
        }

        let mut ans = i32::MAX;
        let mut freq = i32::MAX;
        for (i, f) in count.into_iter().enumerate() {
            if f > 0 && f < freq {
                ans = i as i32;
                freq = f;
            }
        }

        ans
    }
}