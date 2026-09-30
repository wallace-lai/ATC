
struct NeighborSum {
    n: usize,
    pos: Vec<(i32, i32)>,
    grid: Vec<Vec<i32>>,
    diagonal: Vec<i32>,
    adjacent: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    // 2ms，击败100%
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let len = grid.len();
        let mut pos = vec![(0, 0); len * len];
        for i in 0..grid.len() {
            let row = &grid[i];
            for j in 0..row.len() {
                let num = row[j];
                pos[num as usize] = (i as i32, j as i32);
            }
        }

        Self {
            n: len,
            pos: pos,
            grid: grid,
            diagonal: vec![-1; len * len],
            adjacent: vec![-1; len * len],
        }
    }
    
    fn grid_get(&self, i: i32, j: i32) -> i32 {
        let len = self.n as i32;
        if 0 <= i && i < len && 0 <= j && j < len {
            return self.grid[i as usize][j as usize];
        }

        0
    }

    fn adjacent_sum(&mut self, value: i32) -> i32 {
        if self.adjacent[value as usize] > -1 {
            return self.adjacent[value as usize];
        }

        let (i, j) = self.pos[value as usize];
        let sum = self.grid_get(i + 1, j) +
            self.grid_get(i - 1, j) +
            self.grid_get(i, j + 1) +
            self.grid_get(i, j - 1);
        
        self.adjacent[value as usize] = sum;
        sum
    }
    
    fn diagonal_sum(&mut self, value: i32) -> i32 {
        if self.diagonal[value as usize] > -1 {
            return self.diagonal[value as usize];
        }

        let (i, j) = self.pos[value as usize];
        let sum = self.grid_get(i - 1, j - 1) +
            self.grid_get(i + 1, j + 1) +
            self.grid_get(i - 1, j + 1) +
            self.grid_get(i + 1, j - 1);

        self.diagonal[value as usize] = sum;
        sum
    }
}
