struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut sum = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for &s in &salary {
            sum += s;
            if s < min { min = s; }
            if s > max { max = s; }
        }

        (sum - min - max) as f64 / (salary.len() as f64 - 2.0)
    }
}