struct Solution;

impl Solution {
    pub fn master_mind(solution: String, guess: String) -> Vec<i32> {
        let mut ans = vec![0, 0];
        let mut sv = vec![Vec::with_capacity(4); 4];
        let mut gv = vec![Vec::with_capacity(4); 4];

        #[inline]
        fn c2i(c: u8) -> usize {
            match c {
                b'R' => 0,
                b'Y' => 1,
                b'G' => 2,
                b'B' => 3,
                _ => unreachable!(),
            }
        }

        let s = solution.as_bytes();
        let g = guess.as_bytes();
        for (i, (&cs, &cg)) in s.iter().zip(g.iter()).enumerate() {
            if cs == cg {
                ans[0] += 1;
                continue;
            }
            sv[c2i(cs)].push(i);
            gv[c2i(cg)].push(i);
        }

        for (i, j) in sv.iter().zip(gv.iter()) {
            ans[1] += std::cmp::min(i.len(), j.len()) as i32;
        }

        ans
    }
}