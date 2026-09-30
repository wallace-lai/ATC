struct Solution;

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.windows(2).all(|s| { s[0] == s[1] }) {
            return "Flush".to_string();
        }

        let mut count = [0_u8; 16];
        for rank in ranks {
            count[rank as usize] += 1;
        }

        let mut max_cnt = 0;
        for cnt in count {
            if cnt > max_cnt {
                max_cnt = cnt;
            }
        }
        if max_cnt >= 3 {
            return "Three of a Kind".to_string();
        } else if max_cnt == 2 {
            return "Pair".to_string();
        }
    
        "High Card".to_string()
    }
}