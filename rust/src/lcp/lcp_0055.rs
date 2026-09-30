struct Solution;

impl Solution {
    // 14ms，击败100%
    pub fn get_minimum_time(time: Vec<i32>, fruits: Vec<Vec<i32>>, limit: i32) -> i32 {
        let mut sum = 0;
        for pick in fruits {
            // println!("{:?}", pick);
            let t = (pick[1] + limit - 1) / limit;
            sum += time[pick[0] as usize] * t;

            // println!("time : {}, t : {t}", time[pick[0] as usize]);
        }

        sum
    }
}