struct TripleInOne {
    data: Vec<i32>,
    stack_size: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TripleInOne {

    fn new(stackSize: i32) -> Self {
        let size = stackSize as usize;
        Self {
            // 三个栈的top值保存在数组末尾
            // 定位top值：stack_size * 3 + i
            // 定位val值：i * stack_size + top
            data: vec![0; size * 3 + 3],
            stack_size: size,
        }
    }
    
    fn push(&mut self, stack_num: i32, value: i32) {
        let i = stack_num as usize;
        let topi = self.stack_size * 3 + i;
        let top = self.data[topi] as usize;

        if top < self.stack_size {
            self.data[i * self.stack_size + top] = value;
            self.data[topi] += 1;
        }
    }
    
    fn pop(&mut self, stack_num: i32) -> i32 {
        let i = stack_num as usize;
        let topi = self.stack_size * 3 + i;
        let top = self.data[topi] as usize;

        if top == 0 { return -1; }

        self.data[topi] -= 1;
        let new_top = (top - 1) as usize;
        self.data[i * self.stack_size + new_top]
    }
    
    fn peek(&self, stack_num: i32) -> i32 {
        let i = stack_num as usize;
        let topi = self.stack_size * 3 + i;
        let top = self.data[topi] as usize;

        if top == 0 { return -1; }
        self.data[i * self.stack_size + top - 1]
    }
    
    fn is_empty(&self, stack_num: i32) -> bool {
        let i = stack_num as usize;
        self.data[self.stack_size * 3 + i] == 0
    }
}
