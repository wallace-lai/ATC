struct Solution;

impl Solution {
    pub fn make(m: &mut Vec<i32>, ci: &Vec<i32>) {
        for j in 0..m.len() {
            m[j] -= ci[j];
        }
    }

    pub fn unmake(m: &mut Vec<i32>, ci: &Vec<i32>) {
        for j in 0..m.len() {
            m[j] += ci[j];
        }
    }

    pub fn can_make(m: &Vec<i32>, ci: &Vec<i32>) -> bool {
        for j in 0..m.len() {
            if m[j] < ci[j] {
                return false;
            }
        }

        true
    }

    pub fn dfs(m: &mut Vec<i32>, c: &Vec<Vec<i32>>, a: &Vec<Vec<i32>>, l: i32,
        idx: usize, x: &mut i32, y: &mut i32, ans: &mut i32) {
        
        if idx >= c.len() {
            return;
        }

        // 对于第idx个料理，不制作它
        Self::dfs(m, c, a, l, idx + 1, x, y, ans);

        // 对于第idx个料理，制作它
        let ci = &c[idx];
        let ai = &a[idx];
        if Self::can_make(m, ci) {
            // 食材数量减少，美味度和饱腹感增加
            Self::make(m, ci);
            *x += ai[0];
            *y += ai[1];
            if *y >= l {
                *ans = (*ans).max(*x);
            }

            Self::dfs(m, c, a, l, idx + 1, x, y, ans);

            *y -= ai[1];
            *x -= ai[0];
            Self::unmake(m, ci);
        }         
    }

    // 1ms，击败100%
    pub fn perfect_menu(mut m: Vec<i32>, c: Vec<Vec<i32>>, a: Vec<Vec<i32>>, l: i32) -> i32 {
        let mut x = 0;  // 美味度
        let mut y = 0;  // 饱腹感
        let mut ans = -1;

        Self::dfs(&mut m, &c, &a, l, 0, &mut x, &mut y, &mut ans);
        ans
    }
}