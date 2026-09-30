struct Solution;

impl Solution {
    // 超时
    // pub fn breakfast_number(mut staple: Vec<i32>, mut drinks: Vec<i32>, x: i32) -> i32 {
    //     staple.sort_unstable();
    //     drinks.sort_unstable();

    //     let mut ans = 0_u64;
    //     for i in 0..staple.len() {
    //         if staple[i] >= x { break; }
    //         for j in 0..drinks.len() {
    //             if staple[i] + drinks[j] > x { break; }
    //             ans += 1;
    //         }
    //     }

    //    (ans % 1000000007) as i32
    // }

    // 18ms，击败100%
    pub fn breakfast_number(mut staple: Vec<i32>, mut drinks: Vec<i32>, x: i32) -> i32 {
        staple.sort_unstable();
        drinks.sort_unstable();

        let mut ans = 0_u64;
        let mut left = staple.len() as i32 - 1;
        let mut right = 0;
        while left >= 0 && right < drinks.len() as i32 {
            if staple[left as usize] + drinks[right as usize] <= x {
                ans += (left + 1) as u64;
                right += 1;
            } else {
                left -= 1;
            }
        }

        (ans % 1000000007) as i32
    }
}