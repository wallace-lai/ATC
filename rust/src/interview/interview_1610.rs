struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：正常统计（性能差）
    pub fn max_alive_year(birth: Vec<i32>, death: Vec<i32>) -> i32 {
        // 1900 到 2000 年间的存活人数
        let mut count = vec![0; 128];
        for (&b, &d) in birth.iter().zip(death.iter()) {
            for y in b..=d {
                count[y as usize - 1900] += 1;
            }
        }

        let mut ans = 0;
        for (i, &c) in count.iter().enumerate() {
            if c > count[ans] { ans = i; }
        }

        ans as i32 + 1900
    }

    // 法二：差分数组
    pub fn max_alive_year(birth: Vec<i32>, death: Vec<i32>) -> i32 {
        const BEG_YEAR: i32 = 1900;
        const END_YEAR: i32 = 2000;
        const N: usize = (END_YEAR - BEG_YEAR + 2) as usize;

        let mut diff = vec![0; N];
        for (&b, &d) in birth.iter().zip(death.iter()) {
            let bi = (b - BEG_YEAR) as usize;
            let di = (d - BEG_YEAR) as usize;
            diff[bi] += 1;
            if di + 1 < N { diff[di + 1] -= 1; }
        }

        let mut alive = 0;
        let mut max_alive = -1;
        let mut best_year = BEG_YEAR;

        for (i, &d) in diff.iter()
            .enumerate().take((END_YEAR - BEG_YEAR + 1) as usize) {
            alive += d;
            if alive > max_alive {
                max_alive = alive;
                best_year = BEG_YEAR + i as i32;
            }
        }

        best_year
    }
}