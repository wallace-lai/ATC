struct Solution;

impl Solution {
    // 1ms
    // pub fn fizz_buzz(n: i32) -> Vec<String> {
    //     let mut ans = Vec::new();
    //     for i in 1..(n + 1) {
    //         let t3 = if i % 3 == 0 { 1 } else { 0 };
    //         let t5 = if i % 5 == 0 { 1 } else { 0 };
    //         match (t3, t5) {
    //             (1, 1) => { ans.push("FizzBuzz".to_string()); },
    //             (1, 0) => { ans.push("Fizz".to_string()); },
    //             (0, 1) => { ans.push("Buzz".to_string());},
    //             _ => { ans.push(i.to_string()); }
    //         }
    //     }

    //     ans
    // }

    // 性能优化 - 0ms
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = Vec::with_capacity(n as usize);
        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                ans.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                ans.push("Fizz".to_string());
            } else if i % 5 == 0 {
                ans.push("Buzz".to_string());
            } else {
                ans.push(i.to_string());
            }
        }

        ans
    }
}