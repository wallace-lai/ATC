struct Solution;

impl Solution {
    pub fn is_win(x: u8, b: [[u8; 3]; 3]) -> bool {
        // rows
        (b[0][0] == x && b[0][0] == b[0][1] && b[0][1] == b[0][2]) ||
        (b[1][0] == x && b[1][0] == b[1][1] && b[1][1] == b[1][2]) ||
        (b[2][0] == x && b[2][0] == b[2][1] && b[2][1] == b[2][2]) ||
        // cols
        (b[0][0] == x && b[0][0] == b[1][0] && b[1][0] == b[2][0]) ||
        (b[0][1] == x && b[0][1] == b[1][1] && b[1][1] == b[2][1]) ||
        (b[0][2] == x && b[0][2] == b[1][2] && b[1][2] == b[2][2]) ||
        // diag
        (b[0][0] == x && b[0][0] == b[1][1] && b[1][1] == b[2][2]) ||
        (b[2][0] == x && b[2][0] == b[1][1] && b[1][1] == b[0][2])
    }

    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut b: [[u8; 3]; 3] = [[0, 0, 0]; 3];
        for i in 0..moves.len() {
            let x = moves[i][0] as usize;
            let y = moves[i][1] as usize;
            if i & 1 == 0 {
                b[x][y] = b'X';
            } else {
                b[x][y] = b'O';
            }
        }

        if Self::is_win(b'X', b) { return "A".to_string(); }
        if Self::is_win(b'O', b) { return "B".to_string(); }

        let sum = b.iter()
            .flatten()
            .filter(|&p| *p != 0)
            .fold(0, |acc, _| acc + 1);
        if sum == 9 { return "Draw".to_string(); }

        "Pending".to_string()
    }
}