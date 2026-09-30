struct Solution;

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let n = a.len() as i32;

        for dx in (1 - n)..n {
            for dy in (1 - n)..n {
                let mut count = 0;
                let x_beg = std::cmp::max(0 - dx, 0);
                let x_end = std::cmp::min(n - dx, n);
                let y_beg = std::cmp::max(0 - dy, 0);
                let y_end = std::cmp::min(n - dy, n);
                for x in x_beg..x_end {
                    for y in y_beg..y_end {
                        // println!("(x, y) is ({x}, {y}), (x + dx, y + dy) is ({}, {})", x + dx, y + dy);
                        count += a[x as usize][y as usize] *
                            b[(x + dx) as usize][(y + dy) as usize];
                    }
                }

                ans = std::cmp::max(ans, count);
            }
        }

        ans
    }
}