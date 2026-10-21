struct SortedStack {
    stk: Vec<i32>,
    tmp: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SortedStack {

    fn new() -> Self {
        Self {
            stk: Vec::with_capacity(16),
            tmp: Vec::with_capacity(16),
        }
    }
    
    fn push(&mut self, val: i32) {
        // 将所有比val小的元素存放到tmp中
        while !self.stk.is_empty() && self.stk.last().unwrap() < &val {
            self.tmp.push(self.stk.pop().unwrap());
        }

        // 插入val并将tmp中存放的值重新push
        self.stk.push(val);
        while let Some(t) = self.tmp.pop() {
            self.stk.push(t);
        }
    }
    
    fn pop(&mut self) {
        self.stk.pop();
    }
    
    fn peek(&self) -> i32 {
        if self.stk.is_empty() {
            -1
        } else {
            *self.stk.last().unwrap()
        }
    }
    
    fn is_empty(&self) -> bool {
        self.stk.is_empty()
    }
}