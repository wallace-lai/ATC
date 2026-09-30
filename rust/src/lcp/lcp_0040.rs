struct Solution;

impl Solution {
    // 39ms，击败100%
    pub fn maximum_score(mut cards: Vec<i32>, k: i32) -> i32 {
        // 降序排列
        cards.sort_unstable_by(|a, b| b.cmp(a));
        // println!("cards : {:?}", cards);

        let cnt = k as usize;
        let mut sum = 0;
        let mut odd_min = i32::MAX;
        let mut even_min = i32::MAX;
        for i in 0..cnt {
            if cards[i] & 1 == 1 {
                odd_min = odd_min.min(cards[i]);
            } else {
                even_min = even_min.min(cards[i]);
            }
            sum += cards[i];
        }
        if sum & 1 == 0 {
            return sum;
        }

        let mut ans = 0;
        for i in cnt..cards.len() {
            if cards[i] & 1 == 1 {
                // 奇数
                if even_min != i32::MAX {
                    ans = ans.max(sum - even_min + cards[i]);
                }
            } else {
                // 偶数
                if odd_min != i32::MAX {
                    ans = ans.max(sum - odd_min + cards[i]);
                }
            }
        }

        ans

    }
}