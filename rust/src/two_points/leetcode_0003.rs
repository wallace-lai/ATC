struct Solution;

impl Solution {
    // 版本一：7ms
    // pub fn length_of_longest_substring(s: String) -> i32 {
    //     let n = s.len();
    //     let b = s.as_bytes();

    //     let mut ans = 0;
    //     let mut left = 0;
    //     let mut count = [0; 256];

    //     for right in 0..n {
    //         let push = b[right];
    //         count[push as usize] += 1;

    //         while count[push as usize] > 1 {
    //             let pop = b[left];
    //             left += 1;
    //             count[pop as usize] -= 1;
    //         }

    //         ans = ans.max(right - left + 1);
    //     }

    //     ans as i32
    // }

    // 优化版本二：3ms
    pub fn length_of_longest_substring(s: String) -> i32 {
        let b = s.as_bytes();
        let mut ans = 0;
        let mut left = 0;
        // 0表示未出现，否则保存 字符上次出现的位置 + 1
        // 即上次出现位置的后面用于left指针的跳转
        let mut last = [0; 256];

        for (right, &ch) in b.iter().enumerate() {
            let idx = ch as usize;
            let prev = last[idx];

            // 如果该字符上次出现在当前窗口内，left直接跳转到它后面
            if prev > left { left = prev; }

            last[idx] = right + 1;
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}

