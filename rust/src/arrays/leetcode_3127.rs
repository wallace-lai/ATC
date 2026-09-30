struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let n = 3;
        for r in 0..n {
            if r + 2 > n { break; }
            for c in 0..n {
                if c + 2 > n { break; }
                let mut black = 0;
                let mut white = 0;
                for x in r..r+2 {
                    for y in c..c+2 {
                        if grid[x][y] == 'B' {
                            black += 1;
                        } else {
                            white += 1;
                        }
                    }
                }
                if std::cmp::min(black, white) <= 1 {
                    return true;
                }
            }
        }
        false
    }
}