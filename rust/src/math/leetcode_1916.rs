struct Solution;

impl Solution {
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = prev_room.len();
        let mut g = vec![Vec::new(); n];
        let mut fac: i64 = 1;   // 分子
        for i in 1..n {
            fac = fac * (i as i64 + 1) % MOD;
            g[prev_room[i] as usize].push(i as i32);
        }
        // --------------------
        // for r in g.iter() {
            // println!("{:?}", r);
        // }
        // --------------------

        let mut prod: i64 = 1;  // 分母

        fn qpow(mut x: i64, mut n: i64) -> i64 {
            let mut res = 1;
            while n > 0 {
                if n & 1 == 1 {
                    res = res * x % MOD;
                }
                x = x * x % MOD;
                n /= 2;
            }
            res
        }

        fn dfs(g: &Vec<Vec<i32>>, x: i32, prod: &mut i64) -> i32 {
            let mut size = 1;
            for y in &g[x as usize] {
                size += dfs(g, *y, prod);
            }
            *prod = *prod * size as i64 % MOD;
            size
        }

        dfs(&g, 0, &mut prod);
        let ans = fac * qpow(prod, MOD - 2) % MOD;
        ans as i32
    }
}