struct Solution;

impl Solution {
    pub fn num_rook_captures(b: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;

        fn find_rpos(b: &Vec<Vec<char>>) -> (usize, usize) {
            for i in 0..8 {
                for j in 0..8 {
                    if b[i][j] == 'R' {
                        return (i, j);
                    }
                }
            }
            unreachable!()
        }

        let rpos = find_rpos(&b);

        // 上
        let mut x = rpos.0 as i32;
        let mut y = rpos.1 as i32;
        while x - 1 >= 0 {
            match b[x as usize - 1][y as usize] {
                'B' => { break; },
                'p' => { ans += 1; break; },
                _ => { x -= 1; }
            }
        }
        // 下
        x = rpos.0 as i32;
        y = rpos.1 as i32;
        while x + 1 < 8 {
            match b[x as usize + 1][y as usize] {
                'B' => { break; }
                'p' => { ans += 1; break; }
                _ => { x += 1; }
            }
        }
        // 左
        x = rpos.0 as i32;
        y = rpos.1 as i32;
        while y - 1 >= 0 {
            match b[x as usize][y as usize - 1] {
                'B' => { break; }
                'p' => { ans += 1; break; }
                _ => { y -= 1; }
            }
        }
        // 右
        x = rpos.0 as i32;
        y = rpos.1 as i32;
        while y + 1 < 8 {
            match b[x as usize][y as usize + 1] {
                'B' => { break; }
                'p' => { ans += 1; break; }
                _ => { y += 1; }
            }
        }

        ans
    }
}