use std::collections::HashMap;

struct FreqStack {
    freq: HashMap<i32, i32>,        // val --> 当前的频率
    group: HashMap<i32, Vec<i32>>,  // 频率 --> 该频率下元素的栈
    max_freq: i32,                  // 当前的最大频率
}

impl FreqStack {
    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            group: HashMap::new(),
            max_freq: 0
        }
    }
    
    fn push(&mut self, val: i32) {
        // 更新频率
        let f = self.freq.entry(val).or_insert(0);
        *f += 1;
        let f = *f;

        // 压入对应频率的栈
        self.group.entry(f).or_default().push(val);

        // 更新最大频率
        self.max_freq = self.max_freq.max(f);
    }
    
    fn pop(&mut self) -> i32 {
        // 从最大频率的栈顶取出元素
        let stk = self.group.get_mut(&self.max_freq).unwrap();
        let val = stk.pop().unwrap();

        // 如果最大频率栈空了，则最大频率下降
        if stk.is_empty() {
            self.max_freq -= 1;
        }

        // 元素对应频率减1
        *self.freq.get_mut(&val).unwrap() -= 1;

        val
    }
}
