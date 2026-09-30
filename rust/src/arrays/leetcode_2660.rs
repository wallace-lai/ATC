struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut sum1 = 0;
        let mut sum2 = 0;

        fn sum(p: &Vec<i32>) -> i32 {
            let mut ans = 0;
            for i in 0..p.len() {
                ans +=
                    match i {
                        0 => {
                            p[0]
                        },
                        1 => {
                            if p[0] == 10 {
                                p[1] * 2
                            } else {
                                p[1]
                            }
                        },
                        _ => {
                            if p[i - 1] == 10 || p[i - 2] == 10 {
                                p[i] * 2
                            } else {
                                p[i]
                            }
                        }
                    };
            }
            ans
        }

        sum1 += sum(&player1);
        sum2 += sum(&player2);

        if sum1 > sum2 {
            1
        } else if sum1 < sum2 {
            2
        } else {
            0
        }
    }
}