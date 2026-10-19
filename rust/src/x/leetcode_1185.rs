struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        fn is_leap_year(year: i32) -> bool {
            (year % 400 == 0) ||
            (year % 4 == 0 && year % 100 != 0)
        }

        fn days_in_month(year: i32, month: i32) -> i32 {
            match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => if is_leap_year(year) { 29 } else { 28 },
                _ => 0,
            }
        }

        fn days_from_epoch(year: i32, month: i32, day: i32) -> i32 {
            let mut days = 0;

            // 年份在1971到2100之间
            for y in 1971..year {
                days += if is_leap_year(y) { 366 } else { 365 };
            }

            for m in 1..month {
                days += days_in_month(year, m);
            }

            days += day;
            days
        }

        const WEEK: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let d = (days_from_epoch(year, month, day) - 1) % 7;
        let d = (5 + d) as usize % 7;
        // println!("days from epoch is {}", days_from_epoch(year, month, day));
        WEEK[d].to_string()
    }
}