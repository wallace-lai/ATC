struct Solution;

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut vis = vec![false; n];

        let mut d = k;
        let mut i = 0;
        while vis[i] == false {
            vis[i] = true;
            i = (d + i) % n;
            d += k;
        }

        let mut ans = Vec::with_capacity(n);
        for i in 0..vis.len() {
            if vis[i] == false {
                ans.push(i as i32 + 1);
            }
        }

        ans
    }
}