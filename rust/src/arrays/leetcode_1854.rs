struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut year = vec![0; 128];
        for log in logs {
            let birth = log[0];
            let death = log[1];
            for y in birth..death {
                let idx = (y - 1950) as usize;
                year[idx] += 1;
            }
        }

        let mut ans = 0;
        for i in 0..year.len() {
            if year[i] > year[ans] {
                ans = i;
            }
        }

        (ans + 1950) as i32
    }
}