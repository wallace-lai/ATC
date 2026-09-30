struct Solution;

impl Solution {
    // 性能非最优
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {

        fn div_score(nums: &Vec<i32>, n: i32) -> i32 {
            let mut ans = 0;
            for num in nums {
                if num % n == 0 {
                    ans += 1;
                }
            }
            ans
        }

        let mut ans = divisors[0];
        let mut cnt = div_score(&nums, divisors[0]);
        // println!("divisors[0] score is {cnt}");

        for i in 1..divisors.len() {
            let score = div_score(&nums, divisors[i]);
            // println!("divisors[{i}] score is {score}");
            if score > cnt || (score == cnt && divisors[i] < ans) {
                ans = divisors[i];
                cnt = score;
            }
        }

        ans
    }
}