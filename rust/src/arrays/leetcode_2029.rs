struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = [0, 0, 0];
        for val in stones {
            let t = val % 3;
            if t == 0 { cnt[0] += 1; }
            else if t == 1 { cnt[1] += 1; }
            else { cnt[2] += 1; }
        }

        if cnt[0] & 1 == 0 { return cnt[1] >= 1 && cnt[2] >= 1; }

        (cnt[1] - cnt[2] > 2) || (cnt[2] - cnt[1] > 2)
    }
}