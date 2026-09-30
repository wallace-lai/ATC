struct Solution;

impl Solution {
    // 0ms - 击败100%
    // pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    //     let mut ans = 0;
    //     for ops in operations.iter() {
    //         match ops.as_str() {
    //             "++X" => { ans += 1; },
    //             "X++" => { ans += 1; },
    //             "--X" => { ans -= 1; },
    //             "X--" => { ans -= 1; },
    //             _ => {}
    //         }
    //     }

    //     ans
    // }

    // 0ms - 击败100%
    // pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    //     operations.into_iter()
    //         .map(|s| if s.as_bytes()[1] == b'+' { 1 } else { -1 })
    //         .sum()
    // }

    // 0ms
    // pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    //     // +的ASCII值为43，-的ASCII值为45
    //     operations.into_iter()
    //         .map(|s| 44 - s.as_bytes()[1] as i32)
    //         .sum()
    // }

    // 0ms
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        // +的ASCII值为43，-的ASCII值为45
        operations.into_iter()
            .map(|s| (s.as_bytes()[1] as i32 & 2) - 1)
            .sum()
    }
}