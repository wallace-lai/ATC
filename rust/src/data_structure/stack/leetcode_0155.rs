struct MinStack {
    stk1: Vec<i32>,     // 正常栈
    stk2: Vec<i32>,     // 保存当前最小值的栈
}

impl MinStack {

    fn new() -> Self {
        Self {
            stk1: Vec::new(),
            stk2: Vec::new(),
        }
    }
    
    fn push(&mut self, value: i32) {
        self.stk1.push(value);
        match self.stk2.last() {
            Some(&min) if value > min => {
                self.stk2.push(min);
            },
            _ => {
                self.stk2.push(value);
            }
        }
    }
    
    // pop总在非空栈上调用
    fn pop(&mut self) {
        self.stk1.pop();
        self.stk2.pop();
    }
    
    // top总在非空栈上调用
    fn top(&self) -> i32 {
        let t = self.stk1.last().unwrap();
        *t
    }
    
    // getMin总在非空栈上调用
    fn get_min(&self) -> i32 {
        let t = self.stk2.last().unwrap();
        *t
    }
}