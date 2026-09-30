struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let row = strs.len();
        let col = strs[0].len();

        let mut ans = col as i32;
        for j in 0..col {
            for i in 1..row {
                println!("({}, {})", i, j);
                let last_row = strs[i - 1].as_bytes();
                let curr_row = strs[i].as_bytes();
                if last_row[j] > curr_row[j] {
                    println!("{} > {}", last_row[j] as char, curr_row[j] as char);
                    ans -= 1;
                    break;
                }
            }
        }

        ans
    }
}