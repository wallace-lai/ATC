struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let time_segment: Vec<(i32, i32)> = time_series.iter()
            .map(|&i| (i, i + duration))
            .collect();

        let mut total = 0;
        let (mut curr_beg, mut curr_end) = time_segment[0];
        for &(beg, end) in &time_segment[1..] {
            if beg > curr_end {
                // 无重叠，结算前一个时间段
                total += curr_end - curr_beg;
                curr_beg = beg;
                curr_end = end;
            } else {
                // 有重叠，合并终点
                if end > curr_end {
                    curr_end = end;
                }
            }
        }

        total += curr_end - curr_beg;
        total
    }
}