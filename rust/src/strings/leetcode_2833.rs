struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let b = moves.as_bytes();
        let n = b.len();
        let mut lnum = 0;
        let mut rnum = 0;
        let mut _num = 0;

        for i in 0..n {
            match b[i] {
                b'L' => { lnum += 1; },
                b'R' => { rnum += 1; },
                b'_' => { _num += 1; },
                _ => {}
            }
        }

        if lnum >= rnum {
            lnum - rnum + _num
        } else {
            rnum - lnum + _num
        }
    }
}