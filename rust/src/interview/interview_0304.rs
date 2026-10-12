struct MyQueue {
    push_stk: Vec<i32>,
    pop_stk: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            push_stk: Vec::new(),
            pop_stk: Vec::new(),
        }
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.push_stk.push(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if !self.pop_stk.is_empty() {
            // 题目保证不会在空队列上调用pop或者peek
            return self.pop_stk.pop().unwrap();
        }

        while !self.push_stk.is_empty() {
            let n = self.push_stk.len();
            self.pop_stk.push(self.push_stk[n - 1]);
            self.push_stk.pop();
        }

        self.pop_stk.pop().unwrap()
    }
    
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if !self.pop_stk.is_empty() {
            // 题目保证不会在空队列上调用pop或者peek
            let top = self.pop_stk.last().unwrap();
            return *top;
        }

        while !self.push_stk.is_empty() {
            let n = self.push_stk.len();
            self.pop_stk.push(self.push_stk[n - 1]);
            self.push_stk.pop();
        }

        // 题目保证不会在空队列上调用pop或者peek
        let top = self.pop_stk.last().unwrap();
        *top
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.push_stk.is_empty() && self.pop_stk.is_empty()
    }
}
