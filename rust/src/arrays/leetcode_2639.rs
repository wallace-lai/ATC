struct Solution;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![0; n];

        fn width(mut n: i32) -> i32 {
            if n == 0 { return 1; }
            let mut ans = 0;
            if n < 0 {
                ans += 1;
                n = -n;
            }
            while n > 0 {
                ans += 1;
                n /= 10;
            }
            ans
        }

        for y in 0..n {
            let mut tmp = 0;
            for x in 0..m {
                let w = width(grid[x][y]);
                if w > tmp { tmp = w; }
            }
            ans[y] = tmp;
        }

        ans
    }
}