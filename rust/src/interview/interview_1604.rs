struct Solution;

impl Solution {
    #[cfg(any())]
    // 法一：哈希表
    pub fn tictactoe(board: Vec<String>) -> String {
        let n = board.len();

        let mut xpos = HashSet::new();
        let mut opos = HashSet::new();
        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.as_bytes().iter().enumerate() {
                if c == b'X' { xpos.insert((i, j)); }
                else if c == b'O' { opos.insert((i, j)); }
            }
        }

        let check = |pos: &HashSet<(usize, usize)>| {
            // row check
            for i in 0..n {
                let mut ans = true;
                for j in 0..n {
                    if !pos.contains(&(i, j)) {
                        ans = false;
                        break;
                    }
                }
                if ans { return true; }
            }

            // col check
            for j in 0..n {
                let mut ans = true;
                for i in 0..n {
                    if !pos.contains(&(i, j)) {
                        ans = false;
                        break;
                    }
                }
                if ans { return true; }
            }

            // main diag
            let mut ans = true;
            for i in 0..n {
                if !pos.contains(&(i, i)) {
                    ans = false;
                    break;
                }
            }
            if ans { return true; }

            // secondary diag
            ans = true;
            for i in 0..n {
                if !pos.contains(&(i, n - 1 - i)) {
                    ans = false;
                    break;
                }
            }

            ans
        };

        if check(&xpos) {
            return "X".to_string();
        } else if check(&opos) {
            return "O".to_string();
        } else if xpos.len() + opos.len() == n * n {
            return "Draw".to_string();
        } else {
            return "Pending".to_string();
        }
    }

    // 法二：一次扫描 + 计数器
    pub fn tictactoe(board: Vec<String>) -> String {
        let n = board.len();
        // (x计数, o计数)
        let mut rows = vec![(0, 0); n];
        let mut cols = vec![(0, 0); n];
        let mut diag = (0, 0);
        let mut anti = (0, 0);
        let mut filled = 0;

        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.as_bytes().iter().enumerate() {
                match c {
                    b'X' => {
                        rows[i].0 += 1;
                        cols[j].0 += 1;
                        if i == j { diag.0 += 1; }
                        if i + j + 1 == n { anti.0 += 1; }
                        filled += 1;
                    },
                    b'O' => {
                        rows[i].1 += 1;
                        cols[j].1 += 1;
                        if i == j { diag.1 += 1; }
                        if i + j + 1 == n { anti.1 += 1; }
                        filled += 1;
                    },
                    _ => { }
                }
            }
        }

        let x_win = rows.iter().any(|&(x, _)| x == n)
            || cols.iter().any(|&(x, _)| x == n)
            || diag.0 == n
            || anti.0 == n;
        
        let o_win = rows.iter().any(|&(_, o)| o == n)
            || cols.iter().any(|&(_, o)| o == n)
            || diag.1 == n
            || anti.1 == n;

        if x_win {
            "X".to_string()
        } else if o_win {
            "O".to_string()
        } else if filled == n * n {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}