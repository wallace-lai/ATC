struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut f = vec![i32::MIN; n + 1];
        
        f[n] = 0;
        for i in (0..n).rev() {
            let mut pre = 0;
            let mut j = i + 1;
            while j <= i + 3 && j <= n {
                pre += stone_value[j - 1];
                f[i] = f[i].max(pre - f[j]);
                j += 1;
            }
        }

        if f[0] == 0 {
            return "Tie".to_string();
        }

        if f[0] > 0 { "Alice".to_string() } else { "Bob".to_string() }

    }
}