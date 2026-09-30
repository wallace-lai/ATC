struct Solution;

use std::collections::VecDeque;

struct MovingAverage {
    sum: i32,
    size: usize,
    data: VecDeque<i32>,
}

// 7ms，击败100%

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        Self {
            sum: 0,
            size: size as usize,
            data: VecDeque::with_capacity(size as usize),
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        if self.data.len() < self.size {
            self.data.push_back(val);
            self.sum += val;
        } else {
            let first = self.data.pop_front().unwrap();
            self.data.push_back(val);
            self.sum += val - first;
        }

        self.sum as f64 / self.data.len() as f64
    }
}