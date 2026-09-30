struct Solution;

impl Solution {
    // 0ms，击败100%
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut map = vec![[0; 16]; n];
        for p in pick {
            let xi = p[0] as usize;
            let yi = p[1] as usize;
            map[xi][yi] += 1;
        }

        let mut ans = 0;
        for (i, count) in map.into_iter().enumerate() {
            for num in count {
                if num as usize > i {
                    ans += 1;
                    break;
                }
            }
        }

        ans
    }
}