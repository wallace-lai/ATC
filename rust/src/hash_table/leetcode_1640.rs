struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let count = pieces.iter().flatten().fold(0, |acc, _| acc + 1);
        if arr.len() != count as usize { return false; }

        let mut m: HashMap<i32, Vec<i32>> = pieces.into_iter()
            .map(|piece| (piece[0], piece))
            .collect();

        let mut idx = 0;
        while idx < arr.len() {
            let piece = m.entry(arr[idx]).or_insert(vec![]);
            if piece.len() == 0 { return false; }
            if idx + piece.len() > arr.len() { return false; }
            if &arr[idx..idx + piece.len()] != piece.as_slice() {
                return false;
            }

            idx += piece.len();
        }

        true
    }
}