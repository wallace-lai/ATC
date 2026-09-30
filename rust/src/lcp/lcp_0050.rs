use std::i32;

struct Solution;

impl Solution {
    // 7ms，击败100%
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for op in operations {
            let x = op[0] as usize;
            let y = op[1] as usize;
            let give = gem[x] / 2;
            gem[x] -= give;
            gem[y] += give;
        }

        let mut gem_min = i32::MAX;
        let mut gem_max = i32::MIN;
        for i in gem {
            if i > gem_max { gem_max = i; }
            if i < gem_min { gem_min = i; }
        }

        gem_max - gem_min
    }
}