struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let m = m as usize;
        let n = n as usize;
        let mut ans = Vec::new();

        for i in 0..m {
            let mut row = vec![b'#'; n];
            if i == 0 {
                row.fill(b'.');
            } else {
                row[n - 1] = b'.';
            }

            ans.push(unsafe {
                String::from_utf8_unchecked(row)
            });
        }

        ans
    }
}