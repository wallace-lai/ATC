struct MinStack {
    stk: Vec<i32>,
    min_stk: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stk: Vec::new(),
            min_stk: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stk.is_empty() {
            self.min_stk.push(x);
        } else {
            let &min = self.min_stk.last().unwrap();
            self.min_stk.push(if x < min { x } else { min });
        }
        self.stk.push(x);
    }
    
    fn pop(&mut self) {
        self.stk.pop();
        self.min_stk.pop();
    }
    
    fn top(&self) -> i32 {
        *self.stk.last().unwrap_or(&-1)
    }
    
    fn get_min(&self) -> i32 {
        *self.min_stk.last().unwrap_or(&-1)
    }
}
