struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut x = 0;
        let mut y = 0;

        for cmd in &commands {
            match cmd.as_bytes()[0] {
                b'U' => { x -= 1; },
                b'D' => { x += 1; },
                b'L' => { y -= 1; },
                b'R' => { y += 1; },
                _ => {}
            }
        }

        x * n + y
    }
}