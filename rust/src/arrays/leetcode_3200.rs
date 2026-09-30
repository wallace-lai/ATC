struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn max_height(mut v: [i32; 2]) -> i32 {
            let mut level = 1;
            let mut i = 0;

            loop {
                if v[i] < level { break; }
                v[i] -= level;
                level += 1;
                i = (i + 1) % 2;
            }

            level - 1
        }

        std::cmp::max(
            max_height([red, blue]),
            max_height([blue, red])
        )
    }
}