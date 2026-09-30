struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut ans = 0;
        let n = start_time.len();

        for i in 0..n {
            let start = start_time[i];
            let end = end_time[i];
            if start <= query_time && query_time <= end {
                ans += 1;
            }
        }

        ans
    }
}