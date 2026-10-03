struct CustomStack {
    data: Vec<i32>,
    max_size: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(max_size: i32) -> Self {
        let size = max_size as usize;
        Self {
            data: Vec::with_capacity(size),
            max_size: size
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.data.len() >= self.max_size {
            return;
        }
        self.data.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.data.len() == 0 {
            return -1;
        }

        let &top = self.data.last().unwrap();
        self.data.pop();
        top
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let n = std::cmp::min(k as usize, self.data.len());
        for i in 0..n {
            self.data[i] += val;
        }
    }
}
