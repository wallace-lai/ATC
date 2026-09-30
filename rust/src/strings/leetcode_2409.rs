struct Solution;

impl Solution {
    const DAYS_OF_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    pub fn get_days(m: i32, d: i32) -> i32 {
        let mut days = 0;
        for i in 0..(m - 1) {
            days += Self::DAYS_OF_MONTH[i as usize];
        }

        days += d;
        days
    }

    pub fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
        let beg1 = arrive_alice.as_bytes();
        let end1 = leave_alice.as_bytes();
        let beg2 = arrive_bob.as_bytes();
        let end2 = leave_bob.as_bytes();

        let beg_max = std::cmp::max(beg1, beg2);
        let end_min = std::cmp::min(end1, end2);
        if beg_max > end_min { return 0; }
        // println!("beg_max is {:?}", beg_max);
        // println!("end_min is {:?}", end_min);

        let beg_month = (beg_max[0] - b'0') as i32 * 10 +
            (beg_max[1] - b'0') as i32;
        let beg_day = (beg_max[3] - b'0') as i32 * 10 +
            (beg_max[4] - b'0') as i32;

        let end_month = (end_min[0] - b'0') as i32 * 10 +
            (end_min[1] - b'0') as i32;
        let end_day = (end_min[3] - b'0') as i32 * 10 +
            (end_min[4] - b'0') as i32;
        
        Self::get_days(end_month, end_day) - Self::get_days(beg_month, beg_day) + 1
    }
}